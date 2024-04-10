use figment::{providers::{Format, Toml}, Figment};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub enum IngesterConfig {
    RpcBlockSubscription { wss_rpc_url: String },
    //--- below not implemented yet
    GrpcSub,      //not implemented
    WebsocketSub, //not implemented
}

#[derive(Debug, Deserialize, Clone)]
pub enum TransactionSenderConfig {
    Rpc { rpc_url: String },
    //--- below not implemented yet
    Tpu,
}

#[derive(Debug, Deserialize, Clone)]
pub enum SignerConfig {
    KeypairFile { path: String }, //--- below not implemented yet maybe hsm, signer server or some weird sig agg shiz
}

#[derive(Debug, Deserialize, Clone)]
pub struct ProverNodeConfig {
  pub env: Option<String>,
  #[serde(default = "default_bonsol_program")]
  pub bonsol_program: String,
  #[serde(default = "default_risc0_image_folder")]
  pub risc0_image_folder: String,
  #[serde(default = "default_max_image_size_mb")]
  pub max_image_size_mb: u32,
  #[serde(default = "default_image_compression_ttl_hours")]
  pub image_compression_ttl_hours: u32,
  #[serde(default = "default_max_input_size_mb")]
  pub max_input_size_mb: u32,
  #[serde(default = "default_image_download_timeout_secs")]
  pub image_download_timeout_secs: u32,
  #[serde(default = "default_input_download_timeout_secs")]
  pub input_download_timeout_secs: u32,
  #[serde(default = "default_maximum_concurrent_proofs")]
  pub maximum_concurrent_proofs: u32,
  #[serde(default = "default_ingester_config")]
  pub ingester_config: IngesterConfig,
  #[serde(default = "default_transaction_sender_config")]
  pub transaction_sender_config: TransactionSenderConfig,
  #[serde(default = "default_signer_config")]
  pub signer_config: SignerConfig,
  #[serde(default="default_capacity_config")]
  pub capacity_config: CapacityConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CapacityConfig {
  pub max_inflight_proofs: u32,

}

fn default_capacity_config() -> CapacityConfig {
  CapacityConfig {
    max_inflight_proofs: 100,
    //support for other capacity configs pending
  }
}

fn default_bonsol_program() -> String {
  "BoNSrwTtTM4PRkbbPvehk1XzHC65cKfdNSod9FyTejRn".to_string()
}

fn default_risc0_image_folder() -> String {
  "./elf".to_string()
}

fn default_max_image_size_mb() -> u32 {
  10
}

fn default_image_compression_ttl_hours() -> u32 {
  5
}

fn default_max_input_size_mb() -> u32 {
  1
}

fn default_image_download_timeout_secs() -> u32 {
  120
}

fn default_input_download_timeout_secs() -> u32 {
  30
}

fn default_maximum_concurrent_proofs() -> u32 {
  100
}

fn default_ingester_config() -> IngesterConfig {
  IngesterConfig::RpcBlockSubscription {
    wss_rpc_url: "ws://localhost:8900".to_string(),
  }
}

fn default_transaction_sender_config() -> TransactionSenderConfig {
  TransactionSenderConfig::Rpc {
    rpc_url: "http://localhost:8899".to_string(),
  }
}

fn default_signer_config() -> SignerConfig {
  SignerConfig::KeypairFile {
    path: "./node-keypair.json".to_string(),
  }
}

impl Default for ProverNodeConfig {
  fn default() -> Self {
    ProverNodeConfig {
      env: Some("dev".to_string()),
      bonsol_program: default_bonsol_program(),
      risc0_image_folder: default_risc0_image_folder(),
      max_image_size_mb: default_max_image_size_mb(),
      image_compression_ttl_hours: default_image_compression_ttl_hours(),
      max_input_size_mb: default_max_input_size_mb(),
      image_download_timeout_secs: default_image_download_timeout_secs(),
      input_download_timeout_secs: default_input_download_timeout_secs(),
      maximum_concurrent_proofs: default_maximum_concurrent_proofs(),
      ingester_config: default_ingester_config(),
      transaction_sender_config: default_transaction_sender_config(),
      signer_config: default_signer_config(), 
      capacity_config: default_capacity_config(),
    }
  }
}

pub fn load_config(config_path: &str) -> ProverNodeConfig {
  let figment = Figment::new()
  .merge(Toml::file(config_path));
  figment.extract().unwrap()
  
}