# Solana PoCs

This repository is intended to collect different proofs of concept related with Solana questions, along with the necessary code to demonstrate them.

# How
## Create a new PoC candidate
1. Discuss your PoC idea with the team
2. If the PoC idea is accepted, add it to the [list of PoCs](#list-of-pocs), sorted alphabetically. The format is:
   ```
   - [ ] [poc-tag] PoC description. 
   ``` 
3. From now on, anyone can try creating a PoC code.

## Work on the new PoC
1. Create a new branch with the poc tag as name, forked from main
2. Copy the folder `template-folder` and rename it with the poc tag
3. Use the example program that is inside the folder to develop your PoC
4. Upload your changes to the repository periodically
5. Once the code is finished, proceed to show it in the Colloquium meeting
6. If the PoC is correctly executed, mark the poc in the [list of PoCs](#list-of-pocs) as checked.
   ```
   - [x] [poc-tag] PoC description. 
   ```
7. Create a merge request. Be careful to assign the reviewers correctly

# List of PoCs
- [x] [account-close] Closing an account in Solana by setting the lamports balance to 0.
- [ ] [account-close-2] Closing an account in Solana by setting the lamports balance to (rent_value-1).
- [x] [borsh-serialization] Storing and reading by serializing/deserializing data with Borsh.
- [ ] [cpi-is-signer] Checking the `is_signer` boolean is respected between CPI calls.
- [x] [pda-account-creation] Creating a PDA account with an owner different than the "correct" owner (a program deriving it).
