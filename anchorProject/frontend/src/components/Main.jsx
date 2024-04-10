import React, { useEffect } from "react";
import { useWallet } from "@solana/wallet-adapter-react";
import { Connection } from "@solana/web3.js";
import { Program, AnchorProvider, web3,BN } from "@project-serum/anchor";
import idl from "../idl.json";
import { useState } from "react";
import Box from '@mui/material/Box';
import Container from '@mui/material/Container';
import Grid from '@mui/material/Grid';
import Navbar from "./Navbar";
import VoteOption from "./VoteOption";
import VoteTally from "./VoteTally";
import Footer from "./Footer";
import Intro from "./Intro";
import { useSnackbar } from "notistack";
import VoteHistory from "./VoteHistory";
import { preflightCommitment, programID, capitalize } from "../utils";
const BigNumber = require('bignumber.js');

export default function Main({ network, voteAccount }) {
  const { enqueueSnackbar } = useSnackbar();
  const wallet = useWallet();
  const [apiData,setApiData] = useState(null);

  const [votes, setVotes] = useState({
    crunchy: "0",
    smooth: "0",
  });
  const [voteTxHistory, setVoteTxHistory] = useState([]);
  const [response, setResponse] = useState(null);
  const [postTrigger, setPostTrigger] = useState(null);

  useEffect(() => {
    // Call Solana program for vote count
    async function getVotes() {
      const connection = new Connection(network, preflightCommitment);
      const provider = new AnchorProvider(connection, wallet, preflightCommitment);
      const program = new Program(idl, programID, provider);
      try {
        const account = await program.account.voteAccount.fetch(
          voteAccount.publicKey
        );
        setVotes({
          crunchy: (account.crunchy.toString()),
          smooth: (account.smooth.toString()),
        });
      } catch (error) {
        console.log("could not getVotes: ", error);
      }
    }

    if (!!voteAccount) {
      getVotes();
    }
  }, [voteAccount, network, wallet]);

  const handlePostData = async (ballotData) => {
    try {
      const response = await fetch('/decrypt-ballot', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        // Replace this with the actual data you want to send
        body: JSON.stringify({
          ballotData: ballotData,
          // Add more key-value pairs as needed
        }),
      });
      if (!response.ok) {
        throw new Error('Network response was not ok');
      }
      const data = await response.json();
      return data;
    } catch (error) {
      console.error('Error posting data:', error);
      return null;
    }
  };


  async function getProvider() {
    const connection = new Connection(network, preflightCommitment);
    const provider = new AnchorProvider(connection, window.solana, preflightCommitment);
    return provider;
  }

  function logBase(base, value) {
    // BigNumber instances
    const baseBig = new BigNumber(base);
    const valueBig = new BigNumber(value);
  
    // Logarithm change of base formula: log(value) / log(base)
    return valueBig.ln().div(baseBig.ln()); // Using natural logarithm (ln) for change of base
  }

  const descryptBallot = async (ballotData)=>{
    const result = await handlePostData(ballotData);
    return result;
  }

  async function publishVote(){
    const provider = await getProvider();
    const program = new Program(idl, programID, provider);
    const decryptedSmooth = await descryptBallot((votes.smooth));
    const decryptedCrunchy = await descryptBallot((votes.crunchy));
    console.log(decryptedSmooth.decryptedBallotState,decryptedCrunchy.decryptedBallotState)
    try {
      await program.rpc.publishResult(new BN(decryptedCrunchy.decryptedBallotState),new BN(decryptedSmooth.decryptedBallotState),{
        accounts: {
          voteAccount: voteAccount.publicKey,
          user: provider.wallet.publicKey,
          
        },
      });

      const account = await program.account.voteAccount.fetch(
        voteAccount.publicKey
      );

      setVotes({
        crunchy: (account.crunchy.toString()),
        smooth: (account.smooth.toString()),
      });
      enqueueSnackbar("Vote account initialized", { variant: "success" });
    } catch (error) {
      console.log("Transaction error: ", error);
      console.log(error.toString());
      enqueueSnackbar(`Error: ${error.toString()}`, { variant: "error" });
    }
  }

  // Initialize the program if this is the first time its launched
  async function initializeVoting() {
    const provider = await getProvider();
    const program = new Program(idl, programID, provider);
    console.log(voteAccount.publicKey,provider.wallet.publicKey)
    const init_value = new BN('73786976294838206465');
    try {
      await program.rpc.initialize(init_value,{
        accounts: {
          voteAccount: voteAccount.publicKey,
          user: provider.wallet.publicKey,
          systemProgram: web3.SystemProgram.programId,
        },
        signers: [voteAccount],
      });

      const account = await program.account.voteAccount.fetch(
        voteAccount.publicKey
      );

      setVotes({
        crunchy: (account.crunchy.toString()),
        smooth: (account.smooth.toString()),
      });
      enqueueSnackbar("Vote account initialized", { variant: "success" });
    } catch (error) {
      console.log("Transaction error: ", error);
      console.log(error.toString());
      enqueueSnackbar(`Error: ${error.toString()}`, { variant: "error" });
    }
  }

  function ERC20Add(userPreviousBalance, userNewBalance) {
    // BigInt is necessary for handling large integers, similar to U256 in Rust.
    const constant = BigNumber("73786976294838206464"); // 2^66

    let normalizedPrevBalance;

    if (constant > userPreviousBalance) {
        normalizedPrevBalance = BigNumber(1);
    } else {
        normalizedPrevBalance = userPreviousBalance.minus(constant);
    }

    const normalizedNewBalance = userNewBalance.minus(constant);

    // Multiply the two normalized values
    // JavaScript's BigInt does not have an overflowing_mul method, but overflow is not a concern with BigInt
    const product = normalizedPrevBalance.times(normalizedNewBalance);

    // Add 2 times the constant because of the property of the encryption function
    // Note: It seems there might be an error in the original Rust code comment. It says to add 2 times the constant,
    // but the code only adds it once. Here, we follow the code.
    const encryptedSum = product.plus(constant);
    
    return encryptedSum.toString();
}


  const calNewVoteValue =  (side) => {
    if (side === "crunchy"){
      let response = ERC20Add(BigNumber(votes.crunchy),BigNumber('73786976294838206471'))
      let newVotes = votes;
      newVotes.crunchy = response;
      setVotes(newVotes)
    }else{
      let response = ERC20Add(BigNumber(votes.smooth),BigNumber('73786976294838206471'))
      let newVotes = votes;
      newVotes.smooth = response;
      setVotes(newVotes)
    }
  }

  // Vote for either crunchy or smooth. Poll for updated vote count on completion
  async function handleVote(side) {
    console.log(votes)
    calNewVoteValue(side);
    console.log(votes);
    const provider = await getProvider();
    const program = new Program(idl, programID, provider);
    try {
      const tx =
        side === "crunchy"
          ? await program.rpc.voteCrunchy(new BN(votes.crunchy),{
              accounts: {
                voteAccount: voteAccount.publicKey,
              },
            })
          : await program.rpc.voteSmooth(new BN(votes.smooth),{
              accounts: {
                voteAccount: voteAccount.publicKey,
              },
            });

      const account = await program.account.voteAccount.fetch(
        voteAccount.publicKey
      );
      setVotes({
        crunchy: account.crunchy.toString(),
        smooth: account.smooth.toString(),
      });
      enqueueSnackbar(`Voted for ${capitalize(side)}!`, { variant: "success" });
      setVoteTxHistory((oldVoteTxHistory) => [...oldVoteTxHistory, tx]);
    } catch (error) {
      console.log("Transaction error: ", error);
      console.log(error.toString());
      enqueueSnackbar(`Error: ${error.toString()}`, { variant: "error" });
    }
  }

  return (
    <Box height="100%" display="flex" flexDirection="column">
      <Box flex="1 0 auto">
        <Navbar />
        <Container>
          <Grid container spacing={3}>
            <Grid item xs={12}>
              <Intro
                votes={votes}
                initializeVoting={initializeVoting}
                publishVote={publishVote}
                programID={programID}
                voteAccount={voteAccount}
              />
            </Grid>
            <Grid item xs={12}>
              <VoteTally votes={votes} />
            </Grid>
            <Grid item xs={6}>
              <VoteOption side="crunchy" handleVote={handleVote} />
            </Grid>
            <Grid item xs={6}>
              <VoteOption side="smooth" handleVote={handleVote} />
            </Grid>
            <Grid item xs={12}>
              <VoteHistory voteTxHistory={voteTxHistory} />
            </Grid>
          </Grid>
        </Container>
      </Box>
      <Footer programID={programID} voteAccount={voteAccount} />
    </Box>
  );
}
