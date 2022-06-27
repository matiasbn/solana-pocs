import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import {
  createWithSeedSync,
  findProgramAddressSync,
} from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { Poc } from "../target/types/poc";
import { expect } from "chai";

describe("poc", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Poc as Program<Poc>;
  const { connection, wallet: signer } = program.provider;

  it("Is initialized!", async () => {
    const user = await anchor.web3.Keypair.generate();
    const initialTransfer = 1 * anchor.web3.LAMPORTS_PER_SOL;
    const recentBlockhash = await connection.getLatestBlockhash();
    const transaction = new anchor.web3.Transaction({
      ...recentBlockhash,
      feePayer: signer.publicKey,
    }).add(
      anchor.web3.SystemProgram.transfer({
        fromPubkey: signer.publicKey,
        toPubkey: user.publicKey,
        lamports: initialTransfer,
      })
    );
    const signedTx = await signer.signTransaction(transaction);
    await anchor.web3.sendAndConfirmRawTransaction(
      connection,
      signedTx.serialize()
    );
    const userBalance = await connection.getBalance(user.publicKey);
    expect(userBalance).to.be.eql(initialTransfer);

    const [proxyEscrowAccountId, bump] = await findProgramAddressSync(
      [Buffer.from("ProxyEscrow"), signer.publicKey.toBuffer()],
      program.programId
    );

    const tx = await program.rpc.createProxyEscrow({
      accounts: {
        proxyEscrow: proxyEscrowAccountId,
        payer: signer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
    });

    console.log("Your transaction signature", tx);

    const { escrowOwner, rewardsLastClaimedAt } =
      await program.account.proxyEscrow.fetch(proxyEscrowAccountId);

    console.log(escrowOwner.toBase58());

    expect(escrowOwner.toBase58()).to.be.eql(signer.publicKey.toBase58());

    // const createAccountInstruction = await program.account.proxyEscrow.createInstruction()
  });
});
