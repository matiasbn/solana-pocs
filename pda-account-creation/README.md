# Closing an account in Solana
## Authors
Matías Barrios (matias.barrios@kudelskisecurity.com)

## Description
In some occasions, we need to use PDAs to interact with other "parameterized" accounts. That account address can be deterministically obtained if the creation rules are already stated in the program functions, which means that in theory, any user can create and own a PDA address without necessarily be a program. This PoC is intended to check if the runtime allows the creation of PDAs with owners different than the "correct" owner.

## Steps to reproduce
1. Derive a PDA address from a `account_1`, without creating the PDA account.
2. Invoke a `create_account` instruction to create the PDA account, but with an `owner` different from `account_1`.
3. Try to create the PDA account from `account_1` and check that it fails.

## How to run
You can run the test with cargo test:
``` shell
cargo test
```
To check the `println!` macros of the test, run the test showing the outputs:
``` shell
cargo test -- --nocapture
```

## Conclusions
1. To execute the `create_account`, both `from_pubkey` and `to_pubkey` should be signers. As `to_pubkey` is a PDA, which by definition is a public key without a private key, then the instruction is going to fail.
2. A second test was added (`test_create_account_with_keypair`) to demonstrate that, by passing a second signature (corresponding to `to_pubkey`) the create account instructions is validated.
