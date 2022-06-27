# PDA account ownership
## Authors
Matías Barrios (matias.barrios@kudelskisecurity.com)

## Description
A client is using a function that uses an account to verify if a user already has claimed some reward for some period by using a `claim_rewards` instruction of a program called `proxy-rewarder`. In order to do that, they save the `now` value into a `ProxyEscrow` account, specifically the `rewards_last_claimed_at`, so next time the user calls the instruction, he can only claim rewards for periods that are older than this value.
<p align="center">
  <img src="images/evidence-1.svg" style="display: block"/>
  <img src="images/evidence-2.svg" style="display: block"/>
</p>

The `ProxyEscrow` account is supposed to be created by an instruction called `new_proxy_escrow` of the `proxy-rewarder` program, but the `claim_rewards` instruction is not checking that the `ProxyEscrow` account is owned by the `proxy-rewarder` program.
<p align="center">
  <img src="images/evidence-4.svg" style="display: block"/>
</p>

Which means that an user can pass a `ProxyEscrow` account that is not owned by the `proxy-rewarder`, so he can pass multiple that has `rewards_last_claimed_at` set to 0, and drain the `TreasuryVault` (an ATA that contains the rewards) by calling `claim_rewards` multiple times.

The only thing that can stop this attack, is the fact that `claim_rewards` instruction modifies the state of the `ProxyEscrow` account, which would require some Account ownership validation of the runtime.

<p align="center">
  <img src="images/evidence-5.svg" style="display: block"/>
  <img src="images/evidence-6.svg" style="display: block"/>
</p>

This PoC is intended to demonstrate that such a function can modify the state of a `ProxyEscrow` account that is owned by the user (a malicious account) and a `ProxyEscrow` that is owned by the `proxy-rewarder` (correctly created), since the runtime will check in both cases that the owner is "signing" the instruction.

## Steps to reproduce
1. Create an Anchor program that includes the next instructions
   1. an instruction, called `create_proxy_escrow`, which creates a `ProxyEscrow` PDA with `[b"ProxyEscrow",escrow_owner]` as seeds. 
   2. an instruction, called `vulnerable_instruction`, which requires a `ProxyEscrow` as an account, with the same constraints as in the first image.
   3. an instruction, called `mint_tokens`, which creates a Mint, a `TreasuryVault` (the program ATA) and a `TreasuryAuthority` (the ATA transfer authority), mints 100 tokens and transfer them to `TreasuryVault`.
   4. an instruction, called `create_user_ata`, which creates `UserTokenAccount` account.
2. Create multiple `ProxyEscrow` account owned by the user.
3. `vulnerable_instruction` checks if `rewards_last_claimed_at` is equal to 0, revert if not
4. `vulnerable_instruction` modifies `rewards_last_claimed_at` to 1
5. transfer 1 token from the `TreasuryVault` to the user ATA

## How to run
You can run the test with cargo test:
```shell
cargo test
```
To check the `println!` macros of the test, run the test showing the outputs:
```shell
cargo test -- --nocapture
```

## Conclusions
1.

## TODO to finish PoC
- [ ] Paste "Steps to reproduce" into the code to show where the steps are executed
- [ ] Cargo fix the code by running `cargo fix`. Fix the imports that cargo fix is going to delete.
- [ ] Trim the dependencies of the Cargo.toml file
- [ ] Change the test function from `test_your_poc` to `test_your_poc_tag` e.g. `test_borsh_serialization`
- [ ] Mark the PoC as finished into the main README.md