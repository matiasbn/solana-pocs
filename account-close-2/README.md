# Closing an account in Solana
## Authors

## Description

## Steps to reproduce
1. Create an account with some `lamports` balance, and a non-zero `data` field.
2. Check the account state.
3. Set the `lamports` balance to (rent_value-1), without modifying the `data` field.
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
1. 
