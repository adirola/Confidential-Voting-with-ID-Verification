const {PublicKey,Connection} = require('@solana/web3.js');
const bs58 = require('bs58');
const axios = require("axios");
require("dotenv").config();
const { CovalentClient } = require("@covalenthq/client-sdk");

const eERC20_ABI = require("./constants/abi/eERC20.json");

const connection = new Connection("https://api.mainnet-beta.solana.com", "confirmed");
const accountPublicKey = new PublicKey('Enter Account PublicKey Here');

const ADDRESS = {
  "0x8274F": "0xFEd1642e18C6Ff92e52d6E55a58525cdd1785608", // scroll sepolia
  "0x5a2": "0xFEd1642e18C6Ff92e52d6E55a58525cdd1785608", // 1442 zkevm testnet
  "0x1389": "0x24D5Ab77888c20430EB92402096882A2C2203c44",
  "0xAA36A7": "0x7cC82f365A448918Ea79e4DcA62ACeA24B0C3894", // sepolia 11155111
  "0x7A69": "0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512" // erc20 locally deployed
};

const tokenContractAddresses = '0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512'
// '0x5FC8d32690cc91D4c39d9d3abcBD16989F875707';

const chainIdToChainName = {
  "0x8274F": "scroll-sepolia-testnet",
  "0x1389": "mantle-testnet",
};

const api = axios.create({
  baseURL: "https://sherlocked.azurewebsites.net/",
  // baseURL: "http://localhost:3000",
});

class FHETransactionBuilder {
  /**
   * FHETransactionBuilder constructor.
   * @param {string} address - The address to use for the transaction.
   * @param {string} chainId - The chain to use for the transaction in hex
   */
  constructor(address, chain) {
    this.address = address;
    this.chainId = chain;
  }

  async setDecryptedBallotState({ signer }) {
    const encryptedBalance = await this.getEncryptedBalance({
      provider: signer,
    });

    // sign a message to prove owner of this.address
    const message = `I want to know my balance in decrypted form for the address ${this.address}`;
    const signature = await signer.signMessage(message);

    // call network of nodes to get decrypted balance
    const resp = await api.post("/decrypt-balance", {
      encryptedBalance,
      signature,
      address: this.address,
    });

    const { decryptedBalance } = resp.data;

    return decryptedBalance;
  }

  async sendTransaction({ ballotIdentifier, userVote, signer }) {
    // call the network of nodes to get encrypted amount
    const vote = userVote.toLowerCase() === "yes" ? 1 : userVote.toLowerCase() === "yes" ? 0 : -1;
    if(vote != -1){
      const resp = await api.post("/encrypt-amount", {
        plainTextVote: vote,
      });

      console.log("encrypted Vote Value", resp.data);
      const { encryptedVote } = resp.data;
      console.log("this is user vote ", userVote);
      console.log("user voted for ", ballotIdentifier )
      console.log("encrypted vote ", encryptedVote);
      // console.log("paruint  vote ", ethers.utils.parseUnits(encryptedVote, 0));
      console.log("this is signer ", signer);
       // ADDRESS[this.chainId]
      const sendTransaction = {
        to:  tokenContractAddresses,
        data: new ethers.utils.Interface(eERC20_ABI.abi).encodeFunctionData(
          "transfer",
          [to, ethers.utils.parseUnits(encryptedAmount, 0)]
        ),
        value: "0",
        // gasLimit: "100000",
      };

      console.log(" this is the amount ", sendTransaction);

      // call the transfer function with to, cipherAmount
      const txn = await signer.sendTransaction(sendTransaction);

      console.log("this is txn", txn);

      return true;
    }else {
      return false
    }   
  }
}

module.exports = FHETransactionBuilder;
