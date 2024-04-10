mod image;

use crate::callback::TransactionSender;
use crate::{config::ProverNodeConfig, util::get_body_max_size};

use self::image::Image;

use anagram_bonsol_schema::{ClaimV1, DeployV1, ExecutionRequestV1, InputType, ProgramInputType};
use ark_bn254::Bn254;
use dashmap::DashMap;
use figment::error;
use futures_util::SinkExt;
use reqwest::{RequestBuilder, Url};
use risc0_binfmt::MemoryImage;
use risc0_zkvm::{Journal, SuccinctReceipt};
use serde::{Deserialize, Serialize};
use solana_rpc_client_api::request;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use std::fs;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{collections::HashMap, str::from_utf8, sync::Arc};
use std::{convert::TryInto, f64::consts::E};
use tokio::sync::Mutex;
use tokio::task::JoinSet;
use tokio::time::{self, Instant};
use tokio_util::codec::{BytesCodec, FramedRead};
use wasmer::wasmparser::{Frame, Payload};
use wasmer::Memory;
use wasmer_wasix::journal;

use {
    crate::types::{BonsolInstruction, ProgramExec},
    ark_groth16::Groth16,
    ark_serialize::CanonicalSerialize,
    risc0_zkvm::{
        compute_image_id, get_prover_server,
        recursion::identity_p254,
        sha::{Digest, Digestible},
        ExecutorEnv, ExecutorImpl, ProverOpts, VerifierContext, ALLOWED_IDS_ROOT,
    },
    tokio::{sync::mpsc::UnboundedSender, task::JoinHandle},
};
use {
    anagram_bonsol_schema::{
        parse_ix_data, ChannelInstruction, ChannelInstructionArgs, ChannelInstructionIxType,
        StatusTypes, StatusV1, StatusV1Args,
    },
    anyhow::Result,
    flatbuffers::FlatBufferBuilder,
};
use {
    risc0_groth16::{docker::stark_to_snark, split_digest},
    risc0_zkvm::CompactReceipt,
    thiserror::Error,
};
type GrothBn = Groth16<Bn254>;

#[derive(Debug, Error)]
pub enum Risc0RunnerError {
    #[error("Empty instruction")]
    EmptyInstruction,
    #[error("Invalid data")]
    InvalidData,
    #[error("Img too large")]
    ImgTooLarge,
    #[error("Image download error")]
    ImageDownloadError(#[from] anyhow::Error),
    #[error("Invalid input type")]
    InvalidInputType,
    #[error("Transaction error")]
    TransactionError(String),
    #[error("Error with proof compression")]
    ProofCompressionError,
    #[error("Error with proof generation")]
    ProofGenerationError,
}
pub enum ClaimStatus {
    Claiming(Signature),
    Accepted,
}

pub struct InflightProof {
    pub execution_id: String,
    pub image_id: String,
    pub status: ClaimStatus,
    pub expiry: u64,
    pub requester: Pubkey,
    pub program_callback: Option<ProgramExec>,
}

#[derive(Debug, Clone)]
pub enum ProgramInput {
    Empty,
    Resolved(ResolvedInput),
    Unresolved(UnresolvedInput),
}

impl ProgramInput {
    fn index(&self) -> u8 {
        match self {
            ProgramInput::Resolved(ri) => ri.index,
            ProgramInput::Unresolved(ui) => ui.index,
            _ => 0,
        }
    }
}

type InflightProofs = Arc<DashMap<String, InflightProof>>;
type InflightProofRef<'a> = &'a DashMap<String, InflightProof>;

type LoadedImageMap = Arc<DashMap<String, Image>>;
type LoadedImageMapRef<'a> = &'a DashMap<String, Image>;

type InputStagingArea = Arc<DashMap<String, Vec<ProgramInput>>>;
type InputStagingAreaRef<'a> = &'a DashMap<String, Vec<ProgramInput>>;

#[derive(Debug, Clone)]
struct UnresolvedInput {
    pub index: u8,
    pub url: Url,
    pub input_type: ProgramInputType,
}

#[derive(Debug, Clone)]
struct ResolvedInput {
    pub index: u8,
    pub data: Vec<u8>,
    pub input_type: ProgramInputType,
}
pub struct Risc0Runner {
    config: Arc<ProverNodeConfig>,
    loaded_images: LoadedImageMap,
    worker_handle: Option<JoinHandle<Result<()>>>,
    txn_sender: Arc<TransactionSender>,
    input_staging_area: InputStagingArea,
    self_identity: Arc<Pubkey>,
    inflight_proofs: InflightProofs,
}

impl Risc0Runner {
    pub async fn new(
        config: ProverNodeConfig,
        self_identity: Pubkey,
        image_dir: String,
        txn_sender: Arc<TransactionSender>,
    ) -> Result<Risc0Runner> {
        let dir = fs::read_dir(image_dir)?;
        let loaded_images = DashMap::new();
        for entry in dir {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                let img = Image::new(entry.path()).await?;
                println!("Loaded image: {}", &img.id);
                loaded_images.insert(img.id.clone(), img);
            }
        }

        Ok(Risc0Runner {
            config: Arc::new(config),
            loaded_images: Arc::new(loaded_images),
            worker_handle: None,
            txn_sender,
            input_staging_area: Arc::new(DashMap::new()),
            self_identity: Arc::new(self_identity),
            inflight_proofs: Arc::new(DashMap::new()),
        })
    }

    pub fn start(&mut self) -> Result<UnboundedSender<BonsolInstruction>> {
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<BonsolInstruction>();
        let loaded_images = self.loaded_images.clone();
        let txn_sender = self.txn_sender.clone();

        let img_client = Arc::new(
            reqwest::Client::builder()
                .timeout(Duration::from_secs(
                    self.config.image_download_timeout_secs as u64,
                ))
                .build()?,
        );
        let input_client = Arc::new(
            reqwest::Client::builder()
                .timeout(Duration::from_secs(
                    self.config.input_download_timeout_secs as u64,
                ))
                .gzip(true)
                .deflate(true)
                .build()?,
        );
        let config = self.config.clone();
        let self_id = self.self_identity.clone();
        let input_staging_area = self.input_staging_area.clone();
        let inflight_proofs = self.inflight_proofs.clone();
        self.worker_handle = Some(tokio::spawn(async move {
            while let Some(bix) = rx.recv().await {
                let txn_sender = txn_sender.clone();
                let loaded_images = loaded_images.clone();
                let config = config.clone();
                let img_client = img_client.clone();
                let input_client = input_client.clone();
                let self_id = self_id.clone();
                let input_staging_area = input_staging_area.clone();
                let inflight_proofs = inflight_proofs.clone();
                tokio::spawn(async move {
                    let bonsol_ix_type = parse_ix_data(&bix.data)?;
                    let result = match bonsol_ix_type.ix_type() {
                        ChannelInstructionIxType::DeployV1 => {
                            eprintln!("Received deploy request");
                            // Download image if node config allows
                            let payload = bonsol_ix_type
                                .deploy_v1_nested_flatbuffer()
                                .ok_or(Risc0RunnerError::EmptyInstruction)?;
                            handle_image_deployment(&config, &img_client, payload, &loaded_images)
                                .await
                        }
                        ChannelInstructionIxType::ExecuteV1 => {
                            eprintln!("Received execution request");
                            // Evaluate the execution request and decide if it should be claimed
                            let payload = bonsol_ix_type
                                .execute_v1_nested_flatbuffer()
                                .ok_or(Risc0RunnerError::EmptyInstruction)?;
                            handle_execution_request(
                                &config,
                                &inflight_proofs,
                                input_client.clone(),
                                &txn_sender,
                                &loaded_images,
                                &input_staging_area,
                                bix.last_known_block,
                                payload,
                                &bix.accounts,
                            )
                            .await
                        }
                        ChannelInstructionIxType::ClaimV1 => {
                            eprintln!("Received deploy request");
                            let payload = bonsol_ix_type
                                .claim_v1_nested_flatbuffer()
                                .ok_or(Risc0RunnerError::EmptyInstruction)?;
                            handle_claim(
                                &config,
                                &self_id,
                                &inflight_proofs,
                                input_client,
                                &txn_sender,
                                &loaded_images,
                                &input_staging_area,
                                payload,
                                &bix.accounts,
                            )
                            .await
                        }

                        _ => {
                            eprintln!("Unknown instruction type");
                            Ok(())
                        }
                    };
                    if result.is_err() {
                        eprintln!("Error: {:?}", result);
                    }
                    result
                });
            }
            Ok(())
        }));
        Ok(tx)
    }

    pub fn stop(&mut self) -> Result<()> {
        self.worker_handle.take().unwrap().abort();
        Ok(())
    }
}

async fn handle_claim<'a>(
    config: &ProverNodeConfig,
    self_identity: &Pubkey,
    in_flight_proofs: InflightProofRef<'a>,
    input_client: Arc<reqwest::Client>,
    transaction_sender: &TransactionSender,
    loaded_images: LoadedImageMapRef<'a>,
    input_staging_area: InputStagingAreaRef<'a>,
    claim: ClaimV1<'a>,
    accounts: &[Pubkey], // need to create cannonical parsing of accounts per instruction type for my flatbuffer model or use shank 
) -> Result<()> {
    eprintln!("Received claim event");
    let claimer = accounts[2];
    let execution_id = claim.execution_id().ok_or(Risc0RunnerError::InvalidData)?;
    if &claimer != self_identity {
        in_flight_proofs.remove(execution_id);
        print!("Claimer is not self, we didnt win the claim.");
        return Ok(());
    }

    let claim_status = in_flight_proofs.remove(execution_id);
    if let Some((_, mut claim)) = claim_status {
        if let ClaimStatus::Claiming(sig) = claim.status {
            claim.status = ClaimStatus::Accepted;
            if let Some(mut image) = loaded_images.get_mut(&claim.image_id) {
                // load image if we shucked it off to disk

                image.load().await?;
                let start = SystemTime::now();
                let since_the_epoch = start.duration_since(UNIX_EPOCH)?.as_secs();
                image.last_used = since_the_epoch;
                let mut inputs = input_staging_area.get_mut(execution_id).unwrap();

                inputs.sort_by(|a, b| a.index().cmp(&b.index()));

                let unresolved_count = inputs
                    .iter()
                    .filter(|i| match i {
                        ProgramInput::Unresolved(_) => true,
                        _ => false,
                    })
                    .count();
                if unresolved_count > 0 {
                    //resolve inputs
                    let mut url_set = JoinSet::new();
                    for input in inputs.iter() {
                        if let ProgramInput::Unresolved(ui) = input {
                            let client = input_client.clone();
                            let mx_input = (config.max_input_size_mb * 1024 * 1024) as usize;
                            // There should be no other un resolved input types
                            if let ProgramInputType::Private = ui.input_type {
                                let pir = PrivateInputRequest {
                                    identity: claimer,
                                    claim_id: execution_id.to_string(),
                                    input_index: ui.index,
                                };
                                let pir_str = serde_json::to_string(&pir)?;
                                let claim_authorization =
                                    transaction_sender.sign_calldata(&pir_str)?;
                                url_set.spawn(download_private_input(
                                    client,
                                    ui.index,
                                    ui.url.clone(),
                                    mx_input,
                                    pir_str,
                                    claim_authorization,
                                ));
                            }
                        }
                    }
                    // one of the huge problems with the claim system is that we are not guaranteed to have
                    // the inputs we need at the time we claim and no way to

                    while let Some(url) = url_set.join_next().await {
                        match url {
                            Ok(Ok(ri)) => {
                                let index = ri.index as usize;
                                eprintln!("Resolved input: {}", index);
                                inputs[index] = ProgramInput::Resolved(ri);
                            }
                            _ => {
                                in_flight_proofs.remove(execution_id);
                                input_staging_area.remove(execution_id);
                                return Ok(());
                            }
                        }
                    }
                }
                drop(inputs);
                // drain the inputs and own them here
                eprintln!("Inputs resolved, generating proof");
                let (eid, inputs) = input_staging_area.remove(execution_id).unwrap();
                let mem_image = image.get_memory_image()?;
                let result: Result<(Journal, CompressedReciept), Risc0RunnerError> =
                    tokio::task::spawn_blocking(move || {
                        let (journal, reciept) = risc0_prove(mem_image, inputs).map_err(|e| {
                            eprintln!("Error generating proof: {:?}", e);
                            Risc0RunnerError::ProofGenerationError
                        })?;
                        let compressed_receipt =
                            risc0_docker_compress_proof(reciept).map_err(|e| {
                                eprintln!("Error compressing proof: {:?}", e);
                                Risc0RunnerError::ProofCompressionError
                            })?;
                        Ok((journal, compressed_receipt))
                    })
                    .await?;

                if let Ok((journal, reciept)) = result {
                    let input_digest =
                        solana_sdk::bs58::encode(&journal.as_ref()[0..32]).into_string();
                    let sig = transaction_sender
                        .submit_proof(  
                            &eid,
                            claim.requester,
                            claim.program_callback,
                            &reciept.proof,
                            &reciept.inputs,
                            &input_digest,
                        )
                        .await
                        .map_err(|e| Risc0RunnerError::TransactionError(e.to_string()))?;
                    eprintln!("Proof submitted: {:?}", sig);
                }
                in_flight_proofs.remove(&eid);
            }
        }
    }
    //relinquish claim
    Ok(())
}

async fn handle_execution_request<'a>(
    config: &ProverNodeConfig,
    in_flight_proofs: InflightProofRef<'a>,
    input_client: Arc<reqwest::Client>,
    transaction_sender: &TransactionSender,
    loaded_images: LoadedImageMapRef<'a>,
    input_staging_area: InputStagingAreaRef<'a>,
    execution_block: u64,
    exec: ExecutionRequestV1<'a>,
    accounts: &[Pubkey],
) -> Result<()> {
    // current naive implementation is to accept everything we have pending capacity for on this node, but this needs work
    let inflight =  in_flight_proofs.len();
    eprintln!(
        "Inflight: {} {}",
        inflight, config.capacity_config.max_inflight_proofs
    );
    if inflight < config.capacity_config.max_inflight_proofs as usize {
        let eid = exec
            .execution_id()
            .map(|d| d.to_string())
            .ok_or(Risc0RunnerError::InvalidData)?;
        let image_id = exec
            .image_id()
            .map(|d| d.to_string())
            .ok_or(Risc0RunnerError::InvalidData)?;
        let expiry = exec.max_block_height();
        let image_compute_estimate = loaded_images.get(&image_id).map(|img| img.size);
        let computable_by = if let Some(ice) = image_compute_estimate {
            // naive compute cost estimate which is YES WE CAN DO THIS in the default amount of time
            println!("Image compute estimate: {}", ice);
            //ensure compute can happen before expiry
            //execution_block + (image_compute_estimate % config.max_compute_per_block) + 1 some bogus calc
            expiry / 2
        } else {
            u64::MAX
        };
        if computable_by < expiry {
            //the way this is done can cause race conditions where so many request come in a short time that we accept
            // them before we change the value of g so we optimistically change to inflight and we will decrement if we dont win the claim

            let inputs = exec.input().ok_or(Risc0RunnerError::InvalidData)?;
            let mut url_set = JoinSet::new();
            //TODO handle input sets
            let input_vec = vec![ProgramInput::Empty; inputs.len()];
            input_staging_area.insert(eid.clone(), input_vec);
            let mx_input = (config.max_input_size_mb * 1024 * 1024) as usize;
            // grab public inputs optimistically
            for (index, input) in inputs.iter().enumerate() {
                let client = input_client.clone();
                let mx_input = mx_input.clone();
                match input.input_type() {
                    InputType::PublicUrl => {
                        let url = input
                            .data()
                            .map(|d| d.bytes())
                            .ok_or(Risc0RunnerError::InvalidData)?;
                        let url = from_utf8(url)?;
                        let url = Url::parse(url)?;
                        url_set.spawn(dowload_public_input(client, index as u8, url, mx_input));
                    }
                    InputType::Private => {
                        let url = input
                            .data()
                            .map(|d| d.bytes())
                            .ok_or(Risc0RunnerError::InvalidData)?;
                        let url = from_utf8(url)?;
                        let url = Url::parse(url)?;
                        let mut isa = input_staging_area.get_mut(&eid).unwrap();
                        isa[index] = ProgramInput::Unresolved(UnresolvedInput {
                            index: index as u8,
                            url,
                            input_type: ProgramInputType::Private,
                        });
                    }
                    InputType::PublicData => {
                        let data = input
                            .data()
                            .map(|d| d.bytes())
                            .ok_or(Risc0RunnerError::InvalidData)?;
                        let data = data.to_vec();
                        let mut isa = input_staging_area.get_mut(&eid).unwrap();
                        isa[index] = ProgramInput::Resolved(ResolvedInput {
                            index: index as u8,
                            data,
                            input_type: ProgramInputType::Public,
                        });
                    }
                    _ => {
                        // not implemented yet / or unknown
                        return Err(Risc0RunnerError::InvalidInputType.into());
                    }
                }
            }
            while let Some(url) = url_set.join_next().await {
                match url {
                    Ok(Ok(ri)) => {
                        let mut isa = input_staging_area.get_mut(&eid).unwrap();
                        isa.push(ProgramInput::Resolved(ri));
                    }
                    _ => {
                        in_flight_proofs.remove(&eid);
                        input_staging_area.remove(&eid);
                        return Ok(());
                    }
                }
            }
            // ADD SOME CRAZY AGRESSIVE RETRYING HERE
            let sig = transaction_sender
                .claim(&eid, accounts[2], computable_by)
                .await
                .map_err(|e| Risc0RunnerError::TransactionError(e.to_string()));
            match sig {
                Ok(sig) => {
                    let callback_program = exec
                        .callback_program_id()
                        .and_then::<[u8; 32], _>(|v| v.bytes().try_into().ok())
                        .map(|v| Pubkey::from(v));
                    let callback = if callback_program.is_some() {
                        Some(ProgramExec {
                            program_id: callback_program.unwrap(),
                            instruction_prefix: exec
                                .callback_instruction_prefix()
                                .map(|v| v.bytes().to_vec())
                                .unwrap_or(vec![0x1]),
                        })
                    } else {
                        None
                    };

                    in_flight_proofs.insert(
                        eid.clone(),
                        InflightProof {
                            execution_id: eid.clone(),
                            image_id: image_id.clone(),
                            status: ClaimStatus::Claiming(sig),
                            expiry: expiry,
                            requester: accounts[0],
                            program_callback: callback,
                        },
                    );
                }
                Err(e) => {
                    eprintln!("Error claiming: {:?}", e);
                    in_flight_proofs.remove(&eid);
                }
            }
        }
    }
    Ok(())
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PrivateInputRequest {
    identity: Pubkey,
    claim_id: String,
    input_index: u8,
}
async fn download_private_input(
    client: Arc<reqwest::Client>,
    index: u8,
    url: Url,
    max_size: usize,
    body: String,
    claim_authorization: String,
) -> Result<ResolvedInput> {
    let resp = client
        .post(url)
        .body(body)
        // Signature of the json payload
        .header("Authorization", format!("Bearer {}", claim_authorization))
        .header("Content-Type", "application/json")
        .send()
        .await?
        .error_for_status()?;
    let byte = get_body_max_size(resp.bytes_stream(), max_size).await?;
    Ok(ResolvedInput {
        index,
        data: byte.to_vec(),
        input_type: ProgramInputType::Private,
    })
}

async fn dowload_public_input(
    client: Arc<reqwest::Client>,
    index: u8,
    url: Url,
    max_size: usize,
) -> Result<ResolvedInput> {
    let resp = client.get(url).send().await?.error_for_status()?;
    let byte = get_body_max_size(resp.bytes_stream(), max_size).await?;
    Ok(ResolvedInput {
        index,
        data: byte.to_vec(),
        input_type: ProgramInputType::Public,
    })
}

async fn handle_image_deployment<'a>(
    config: &ProverNodeConfig,
    http_client: &reqwest::Client,
    deploy: DeployV1<'a>,
    loaded_images: LoadedImageMapRef<'a>,
) -> Result<()> {
    let url = deploy.url().ok_or(Risc0RunnerError::InvalidData)?;
    let size = deploy.size_();
    let resp = http_client.get(url).send().await?.error_for_status()?;
    let min = std::cmp::min(size, (config.max_image_size_mb * 1024 * 1024) as u64) as usize;
    if resp.status().is_success() {
        let stream = resp.bytes_stream();
        let byte = get_body_max_size(stream, min)
            .await
            .map_err(|e| Risc0RunnerError::ImageDownloadError(e))?;
        let img = Image::from_bytes(byte)?;
        loaded_images.insert(img.id.clone(), img);
    }
    Ok(())
}

// proving function, no async this is cpu/gpu intesive
fn risc0_prove(
    memory_image: MemoryImage,
    sorted_inputs: Vec<ProgramInput>,
) -> Result<(Journal, SuccinctReceipt)> {
    let mut env_builder = ExecutorEnv::builder();
    for input in sorted_inputs.into_iter() {
        match input {
            ProgramInput::Resolved(ri) => {
                env_builder.write_slice(&ri.data);
            }
            _ => {
                return Err(Risc0RunnerError::InvalidInputType.into());
            }
        }
    }
    let env = env_builder.build()?;
    let mut exec = ExecutorImpl::new(env, memory_image)?;
    let session = exec.run()?;

    // Obtain the default prover.
    let opts = ProverOpts::default();
    let ctx = VerifierContext::default();
    let prover = get_prover_server(&opts).unwrap();
    let receipt = prover.prove_session(&ctx, &session).unwrap();
    let composite_receipt = receipt.inner.composite().unwrap();
    let succinct_receipt = prover.compress(composite_receipt).unwrap();
    let ident_receipt = identity_p254(&succinct_receipt).unwrap();
    Ok((receipt.journal, ident_receipt))
}
pub struct CompressedReciept {
    pub inputs: Vec<u8>,
    pub proof: Vec<u8>,
}
/// Compresses the proof to be sent to the blockchain
/// This is a temporary solution until the wasm groth16 prover or a rust impl is working
fn risc0_docker_compress_proof(succint_receipt: SuccinctReceipt) -> Result<CompressedReciept> {
    let sealbytes = succint_receipt.get_seal_bytes();
    // to be replaced with non docker thing
    let seal = stark_to_snark(&sealbytes)?;
    let claim = succint_receipt.claim;
    let digest = claim.digest();
    let root = hex::decode(ALLOWED_IDS_ROOT).unwrap();
    let rb: [u8; 32] = root.try_into().unwrap();
    let (i0, i1) = split_digest(digest)?;
    let (c0, c1) = split_digest(Digest::from(rb))?;
    let mut i0v = Vec::with_capacity(32);
    i0.serialize_uncompressed(&mut i0v).unwrap();
    let mut i1v = Vec::with_capacity(32);
    i1.serialize_uncompressed(&mut i1v).unwrap();
    let mut c0v: Vec<_> = Vec::with_capacity(32);
    c0.serialize_uncompressed(&mut c0v).unwrap();
    let mut c1v = Vec::with_capacity(32);
    c1.serialize_uncompressed(&mut c1v).unwrap();
    let mut input_vec = Vec::with_capacity(128);
    c0v.reverse();
    c1v.reverse();
    i0v.reverse();
    i1v.reverse();
    input_vec.extend_from_slice(&c0v);
    input_vec.extend_from_slice(&c1v);
    input_vec.extend_from_slice(&i0v);
    input_vec.extend_from_slice(&i1v);
    Ok(CompressedReciept {
        inputs: input_vec,
        proof: seal.to_vec(),
    })
}
