# Storing and reading by serializing/deserializing data with Borsh
## Authors
Matías Barrios

## Description
The `data` field of an account is just an array of bytes. When we create a new account, we have to serialize the data to store it and deserialize to read it in a way that it makes sense.
This PoC is intended to show what is the process of storing and reading the `data` field of an account by using Borsh serialization/deserialization.

## Steps to reproduce
1. Create a struct with a couple fields in it to store different data.
2. Create an account to store this struct.
3. Cast an instance of the struct with some values.
4. Modify the account content by sending a transaction, using the serialized instance as data.
5. Read the data content and deserialize it with Borsh.
6. Compare the deserialized data with the casted instance.

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
1. Is necessary to use `Instruction::new_with_borsh` to send serialized data into the transaction.
2. Is necessary to initialize the account with enough space to store the serialized data, else the deserialization fails.

