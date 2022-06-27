import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { Poc } from '../target/types/poc';

describe('poc', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Poc as Program<Poc>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
