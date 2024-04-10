var createError = require("http-errors");
var express = require("express");
var cookieParser = require("cookie-parser");
var logger = require("morgan");
var cors = require("cors");
const nacl = require('tweetnacl');
const {PublicKey,Connection} = require('@solana/web3.js');
const bs58 = require('bs58');
const anchor = require("@coral-xyz/anchor");

var app = express();

app.use(logger("dev"));
app.use(express.json());
app.use(express.urlencoded({ extended: false }));
app.use(cookieParser());

const constant = BigInt("73786976294838206464");
const ownerAddress = ""

const voteAccount = anchor.web3.Keypair.generate();
// const connection = new Connection("https://api.mainnet-beta.solana.com", "confirmed");
// const accountPublicKey = new PublicKey('Enter Account PublicKey Here');
const corsOptions = {
  origin: "*",
  credentials: true,
  optionSuccessStatus: 200,
};

app.use(express.json());
app.use(cors(corsOptions));

//app.set('view engine', 'html');

app.get("/", (req, res) => {
  res.send("This is the network of nodes");
});

const logBase7 = (x) => {
  return Math.log(x) / Math.log(7);
};

app.get("/voteAccount", (req, res) => {
  console.log(voteAccount)
  res.json({ voteAccount });
});

app.post("/decrypt-ballot", (req, res) => {
  const { ballotData, signature,message } = req.body;
  console.log(" this is body ", req.body);

  // const messageUint8 = new TextEncoder().encode(message);
  // const signatureUint8 = bs58.decode(signature);
  // const publicKeyUint8 = new PublicKey(ownerAddress).toBytes();
  

  // const isSignatureValid = nacl.sign.detached.verify(messageUint8, signatureUint8, publicKeyUint8);

  // if (!isSignatureValid) {
  //   res.status(401).json({ message: "Invalid signature received you are not the owner" });
  // }

  // try {
  //   // Fetch the account info
  //   const accountInfo = await connection.getAccountInfo(accountPublicKey);
  //   if (accountInfo === null) {
  //     console.log("Account not found");
  //     return;
  //   }
  
  //   console.log("Data length:", accountInfo.data.length);

  //   //fetch ballot state from smart contract
  //   ballotData = 23424;
    
  //   // Example for deserializing data (You need to replace this with actual deserialization based on your contract)
  //   // const deserializedData = MyContractSchema.deserialize(accountInfo.data);
  //   // console.log("Deserialized Data:", deserializedData);

  // } catch (error) {
  //   console.error("Failed to read account data:", error);
  // }

  // decrypt the balance
  const ballotDataNormalise = BigInt(ballotData);

  const ballotState = ballotDataNormalise - constant;
  const decryptedBallotState = logBase7(Number(ballotState));

  res.json({ decryptedBallotState });
});

app.post("/encrypt-vote", (req, res) => {
  console.log("request", req.body);
  // console.log(req)

  const { plainTextVote } = req.body;

  const vote = parseInt(plainTextVote);
  const voteValue = BigInt(String(7 ** vote));
  // encrypt the vote
  const encryptedVote = String(voteValue + constant);
  console.log(" this is ep amt ", encryptedVote);
  res.json({ encryptedVote });
});

// catch 404 and forward to error handler
app.use(function (req, res, next) {
  next(createError(404));
});

// error handler
app.use(function (err, req, res, next) {
  // set locals, only providing error in development
  res.locals.message = err.message;
  res.locals.error = req.app.get("env") === "development" ? err : {};

  // render the error page
  res.status(err.status || 500);
  res.render("error");
});

module.exports = app;
