import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import {
  createWithSeedSync,
  findProgramAddressSync,
} from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { Poc } from "../target/types/poc";
import { expect } from "chai";
import { Signer } from "@solana/web3.js";

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

    await program.rpc.createProxyEscrow({
      accounts: {
        proxyEscrow: proxyEscrowAccountId,
        payer: signer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
    });

    const { escrowOwner, rewardsLastClaimedAt } =
      await program.account.proxyEscrow.fetch(proxyEscrowAccountId);

    expect(escrowOwner.toBase58()).to.be.eql(signer.publicKey.toBase58());

    const userOwnedProxyEscrow = await anchor.web3.Keypair.generate();
    console.log(userOwnedProxyEscrow);

    const createAccountInstruction =
      await program.account.proxyEscrow.createInstruction(userOwnedProxyEscrow);

    const createProxyEscrowTx = new anchor.web3.Transaction({
      ...(await connection.getLatestBlockhash()),
      feePayer: signer.publicKey,
    }).add(createAccountInstruction);

    createProxyEscrowTx.sign({
      publicKey: userOwnedProxyEscrow.publicKey,
      secretKey: userOwnedProxyEscrow.secretKey,
    } as Signer);
    await signer.signTransaction(createProxyEscrowTx);

    await anchor.web3.sendAndConfirmRawTransaction(
      connection,
      createProxyEscrowTx.serialize()
    );

    const userProxyEscrowAccountData = await program.account.proxyEscrow.fetch(
      userOwnedProxyEscrow.publicKey
    );

    console.log(userProxyEscrowAccountData);
  });
});
