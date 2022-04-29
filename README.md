# Solana PoCs

This repository is intended to collect different proofs of concept related with Solana questions, along with the necessary code to demonstrate them.

# How
## Create a new PoC candidate
1. Discuss your PoC idea with the team
2. If the PoC idea is accepted, add it to the [list of PoCs](#list-of-pocs). The format is:
   ```
   - [ ] [poc-tag] PoC description. 
   ``` 
3. From now on, anyone can try creating a PoC code.

## Work on the new PoC
1. Create a new branch with the poc tag as name, forked from main
2. Create a folder with the PoC tag
3. Copy the `poc-template-readme.md` to the new PoC folder and rename it to README.md
4. Complete the new README.md with your PoC information
5. Develop the PoC on your computer
6. Upload your changes to the repository periodically
7. Once the code is finished, proceed to show it in the Colloquium meeting
8. If the PoC is correctly executed, mark the poc in the [list of PoCs](#list-of-pocs) as checked.
   ```
   - [x] [poc-tag] PoC description. 
   ```
9. Create a merge request. Be careful to assign the reviewers correctly

# List of PoCs
- [x] [account-close] Closing an account in Solana by setting the lamports balance to 0.
- [ ] [account-close-2] Closing an account in Solana by setting the lamports balance to (rent_value-1).
- [ ] [cpi-is-signer] Checking the `is_signer` boolean is respected between CPI calls.
