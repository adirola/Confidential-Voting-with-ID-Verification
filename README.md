# EnigmaVote
A system that enables confidential voting on solana blockchain using FHE enabled by zkVM and identity Verification of the voter to avoid fake voting
## Architecture Diagram

![Architecture Diagram](https://github.com/adirola/EnigmaVote/blob/main/staticAssets/architecture.png)

> FHE based privacy enabling solution for Solana Blockchain (ZK + FHE + MPC)

## Problem
The inherent transparency of blockchain technology, while facilitating easy verification and trust among users, also means that every transaction or data mutation is publicly visible, reinforcing the system's integrity and reliability. This very transparency paved the way for the rise of Decentralized Autonomous Organizations (DAOs), which operate based on collective decision-making without centralized authority, often through blockchain-based voting systems. However, the public visibility of these votes within these blockchain based voting system can lead to many challenges such as -

1. Privacy Concerns: The transparency of blockchain transactions exposes users' financial and voting actions to public scrutiny, potentially compromising privacy.
2. Strategic Voting: With votes being visible, members might vote strategically based on how others have voted rather than their true preferences, which can skew the decision-making process.
3. Influence and Pressure: The visibility of votes before the conclusion of voting can lead to undue influence, where voters are swayed by the choices of early voters or those with significant influence within the DAO.
4. Security Risks: Public transactions and voting patterns can be exploited for malicious purposes, such as targeted phishing attacks or manipulation of vote outcomes.
5. Transparency vs. Anonymity: While transparency is crucial for trust and verification, finding the balance between transparency and voter anonymity remains a significant challenge to ensure fair and unbiased voting within DAOs.

On all the above issues, the core problem is the public visibility of the data on the blockchain. EnigmaVote is a full scale developer friendly solution to encrypt this public data, so anyone trying to snoop on the data is unable to make sense out of it.

## Solution
EnigmaVote is a full fledged infrastructure developed from scratch which devs can use to write their custom smart contracts capable of operating on encrypted data over the blockchain.

1. When the user sends the transaction to the smart contract, before calling the function on chain, it is first encrypted by the MPC based encryptor and the encrypted comes to the SDK.
2. SDK then calls the smart contract function with encrypted data as function parameters. Thus the smart contract operates on the encrypted data.
3. Computation on the encrypted data is gas heavy, therefore it is outsourced to zkVM based RISC0 proof computer which computes and provide the proofs to ensure that the operations performed are legit. The proof is then verified by the relayer deployed on the EVM chain and then finally states updated.
4. On chain every computation happened in encrypted domain, the user can decrypt the data later with the help of MPC based decryptor, after proving the ownership of data.

For this hackathon we have demonstrated it over the a simple voting system. We have implemented our own custom voting system and relay contract which stores the users vote in encrypted domain. All operations of voting like mark vote, update vote is done in encrypted domain, which is decrypted by the MPC decryptor to show the exact  value at the end of the ballot.

