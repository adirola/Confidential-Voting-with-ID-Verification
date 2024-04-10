
import './App.css';
import {useEffect,useState,useCallback} from 'react'
import idl from "./idl.json"
import {Connection,PublicKey,clusterApiUrl} from '@solana/web3.js'
import {Program, AnchorProvider, web3,utils,BN} from '@project-serum/anchor'


import {
  ConnectionProvider,
  WalletProvider,
} from "@solana/wallet-adapter-react";
import { WalletDialogProvider } from "@solana/wallet-adapter-material-ui";
import { PhantomWalletAdapter } from "@solana/wallet-adapter-wallets";
import { SnackbarProvider, useSnackbar } from "notistack";
import { createTheme, ThemeProvider, makeStyles } from "@mui/material/styles";
import { blue, orange } from '@mui/material/colors';

import Main from "./components/Main";

const localnet = "http://127.0.0.1:8899";
const network = localnet;

const wallets = [new PhantomWalletAdapter()];

const theme = createTheme({
  palette: {
    primary: {
      main: blue[300],
    },
    secondary: {
      main: orange[300],
    },
  },
  components: {
    MuiButtonBase: {
      styleOverrides: {
        root: {
          justifyContent: 'flex-start',
        },
      },
    },
    MuiButton: {
      styleOverrides: {
        root: {
          textTransform: 'none', // Set to 'none' to achieve the undefined behavior from v4
          padding: '12px 16px',
          fontWeight: 600,
        },
        startIcon: {
          marginRight: '8px',
        },
        endIcon: {
          marginLeft: '8px',
        },
        label: {
          color: 'white',
        },
      },
    },
    MuiLink: {
      styleOverrides: {
        root: {
          color: 'inherit', // 'initial' changed to 'inherit' for better practice, but you could also use 'initial' if that was the intention
        },
      },
    },
  },
});


// const App = () => {
//   const [walletAddress,setWalletAddress] = useState(null);
//   const [voteAccount, setVoteAccount] = useState(null);
//   const programID = new PublicKey(idl.metadata.address);
//   const network = "http://127.0.0.1:8899";
//   const opts = {
//     preflightCommitment : "processed"
//   }
//   const {SystemProgram} = web3;
//   const getProvider = () => {
//     const connection = new Connection(network,opts.preflightCommitment);
//     const provider = new AnchorProvider(connection,window.solana,opts.preflightCommitment);
//     return provider
//   }
//   const checkIsWalletConnected = async() => {
    
//     try{
      
//       const {solana} = window;
//       console.log(solana);
//       if(solana?.isPhantom){
//         console.log('wallet found');
//         const response = await solana.connect({
//           onlyIfTrusted:true
//         });
//         console.log('connected with public key', response.publicKey.toString());
//         setWalletAddress(response.publicKey.toString())
//       }else{
//         alert('install a wallet')
//       }
//     }catch(err){
//       console.log(err)
//     }
   
//   }

//   const connectWallet = async () => {
//     const {solana} = window;
//     if(solana){
//       const response = await solana.connect();
//       console.log('connected with public key', response.publicKey.toString());
//       setWalletAddress(response.publicKey.toString())
//     }else{
//       alert('install a wallet')
//     }
//   }

//   const createVotingAccount = async () => {
//     try{
//       const provider = getProvider();
//       const program = new Program(idl,programID,provider)
//       await program.rpc.initialize({
//         accounts: {
//           voteAccount: voteAccount.publicKey,
//           user: provider.wallet.publicKey,
//           systemProgram: SystemProgram.programId,
//         },
//         signers: [voteAccount],
//       });

//       const account = await program.account.voteAccount.fetch(
//         voteAccount.publicKey
//       );

//       console.log(parseInt(account.crunchy.toString()),parseInt(account.smooth.toString()))
//       console.log('account initialised')

//     }catch(err){
//       console.log(err)
//     }
//   }

//   const renderNotconnectedButton = () => {
//     return <button onClick={connectWallet}>Connect Wallet</button>
//   };
//   const renderConnectedButton = () => {
//     return <button onClick={createVotingAccount}>Create Voting Account</button>
//   };

//   useEffect(()=>{
//     const onLoad = async() =>{
//       await checkIsWalletConnected();
//     }
//     window.addEventListener('load',onLoad);
//     return () => window.removeEventListener('load',onLoad)
//   },[])

//   useEffect(() => {
//     fetch("/voteAccount")
//       .then((response) => response.json())
//       .then((data) => {
//         console.log(data);
//         const accountArray = Object.values(data.voteAccount._keypair.secretKey);
//         const secret = new Uint8Array(accountArray);
//         const kp = web3.Keypair.fromSecretKey(secret);
//         setVoteAccount(kp);
//       })
//       .catch((error) => {
//         console.log(error);
//       });
//   }, [])

//   return <div className='App'>
//     {!walletAddress &&renderNotconnectedButton()}
//   {voteAccount && walletAddress && renderConnectedButton()}</div>
// };

function AppWrappedWithProviders() {
  const { enqueueSnackbar } = useSnackbar();
  const [voteAccount, setVoteAccount] = useState(null);

  useEffect(() => {
    fetch("/voteAccount")
      .then((response) => response.json())
      .then((data) => {
        console.log(data);
        const accountArray = Object.values(data.voteAccount._keypair.secretKey);
        const secret = new Uint8Array(accountArray);
        const kp = web3.Keypair.fromSecretKey(secret);
        setVoteAccount(kp);
      })
      .catch((error) => {
        console.log(error);
        enqueueSnackbar("Could not fetch vote account", { variant: "error" });
      });
  }, [enqueueSnackbar]);

  const onWalletError = useCallback(
    (error) => {
      enqueueSnackbar(
        error.message ? `${error.name}: ${error.message}` : error.name,
        { variant: "error" }
      );
      console.error(error);
    },
    [enqueueSnackbar]
  );

  // Wrap <Main /> within <WalletProvider /> so that we can access useWallet hook within Main
  return (
    <WalletProvider wallets={wallets} onError={onWalletError} autoConnect>
      <WalletDialogProvider>
        <Main network={network} voteAccount={voteAccount} />
      </WalletDialogProvider>
    </WalletProvider>
  );
}

export default function App() {
  return (
    <ThemeProvider theme={theme}>
      <SnackbarProvider>
        <ConnectionProvider endpoint={network}>
          <AppWrappedWithProviders />
        </ConnectionProvider>
      </SnackbarProvider>
    </ThemeProvider>
  );
}
