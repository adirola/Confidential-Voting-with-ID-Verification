import * as anchor from "@coral-xyz/anchor";
const { SystemProgram } = anchor.web3;
const assert = require("assert");
import {Voterelay} from '../target/types/voterelay'
import {VotingSystem} from '../target/types/voting_system'


describe("voting-system", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);
  const voteAccount = anchor.web3.Keypair.generate();
  const voteRelay = anchor.web3.Keypair.generate();

  const program = anchor.workspace.VotingSystem as anchor.Program<VotingSystem>;
  const program2 = anchor.workspace.Voterelay as anchor.Program<Voterelay>;

 

  it("Initializes with 0 votes for crunchy and smooth", async () => {

    console.log("Testing Initialize...");
    
    // The last element passed to RPC methods is always the transaction options
    // Because voteAccount is being created here, we are required to pass it as a signers array
    const init_value = new anchor.BN('73786976294838206465');
    await program.rpc.initialize(init_value,{
      accounts: {
        voteAccount: voteAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [voteAccount],
    });

    const account = await program.account.voteAccount.fetch(
      voteAccount.publicKey
    );
    
    console.log("Crunchy: ", account.crunchy.toString());
    console.log("Smooth: ", account.smooth.toString());
    assert.ok(
      account.crunchy.toString() == init_value.toString() && account.smooth.toString() == init_value.toString()
    );
  });

  it("Votes correctly for crunchy", async () => {
    console.log("Testing voteCrunchy...");
    const val =new anchor.BN('73786976294838206471');
    const init_value = new anchor.BN('73786976294838206465');
    await program.rpc.voteCrunchy(val,{
      accounts: {
        voteAccount: voteAccount.publicKey,
      },
    });
    await program2.rpc.createVote(new anchor.BN('0'),val,{
      accounts: {
        voteRecords: voteRelay.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [voteRelay],
    });

    const account = await program.account.voteAccount.fetch(
      voteAccount.publicKey
    );
    
    const account2 = await program2.account.voteRecords.fetch(
      voteRelay.publicKey
    )
    console.log(account2);
    console.log("Crunchy: ", account.crunchy.toString());
    console.log("Smooth: ", account.smooth.toString());

    assert.ok(
    );
  });
  // it("Votes correctly for smooth", async () => {
  //   console.log("Testing voteSmooth...");
  //   await program.rpc.voteSmooth({
  //     accounts: {
  //       voteAccount: voteAccount.publicKey,
  //     },
  //   });

  //   const account = await program.account.voteAccount.fetch(
  //     voteAccount.publicKey
  //   );
  //   console.log("Crunchy: ", account.crunchy.toString());
  //   console.log("Smooth: ", account.smooth.toString());

  //   assert.ok(
  //     account.crunchy.toString() == 1 && account.smooth.toString() == 1
  //   );
  // });
});




// describe("crunchy-vs-smooth", () => {
//   // Configure the client
//   const provider = anchor.Provider.env();
//   anchor.setProvider(provider);

//   const program = anchor.workspace.CrunchyVsSmooth;
//   const voteAccount = anchor.web3.Keypair.generate();

  
  
// });

