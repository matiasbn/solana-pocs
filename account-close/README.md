# Closing an account in Solana
## Authors
Matías Barrios (matias.barrios@kudelskisecurity.com)

## Description
We discussed with some teammates about the account closing process. In theory, if the account `lamports` field is 0 or smaller than the rent value for an epoch, then the System Program should delete the account in the next epoch.
**Theoretically, this is the only requirement to close an account**. Rez Khan mentioned that might be necessary to zero out the `data` field. This Poc is intended to verify what is the minimum requirements to close an account.

## Steps to reproduce
1. Create an account with some `lamports` balance, and a non-zero `data` field.
2. Check the account state.
3. Withdraw the lamports balance from the account, without modifying the `data` field.
4. Check that account is closed.

## How to run
You can run the test with cargo test:
``` shell
cargo test
```
To check the `println!` macros of the test, run the test showing the outputs:
``` shell
cargo test -- --show-output
```
## Conclusions
1. We demonstrated that when`lamports=0` the account is deleted by the system program, even when the data is not 0.
2. We still have to test the case when `lamports` is slightly smaller than the rent. In theory, it should be deleted too.
3. Is not necessary to trigger an instruction to transfer the lamports balance. In the context of the instruction, the balance of the accounts can be modified inline if the accounts owners are flagged as `signer`.
4. In the instruction context, the system program monitors that the changes in the lamports balances of the total of the accounts equals to 0.  
5. in ocassions, `solana_program_test` takes too long to complete transaction, even having a successful outcome at the end of the transaction (according to log).
