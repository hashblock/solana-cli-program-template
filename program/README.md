# Sample program
The sample program is intended to provide a fundemental frame for your own programs.

The following aspects are demonstrated by the program:
1. Account validation
2. Unpacking the instruction to the program and routing the instruction execution
3. Account state (data) reading and writing
4. Serializing and deserializing to and from account states
5. Custom errors
6. Collecting fees for transaction, or not

## Instructions
There are three (3) primary instructions which are not charged a fee to any accounts by the program:
1. Mint - Mints a key/value pair to an account. Fails if the key already exist in the account
2. Transfer - Transfers a key/value pair from one account to another. Fails if the key does not exist in the "from" account or if the key already exists in the "to" account
3. Burn - Burns (removes) a key/value pair from an account. Fails if the key does not exist in the account

And three (3) fee charging variants:

4. MintWithFee - Same as Mint but debits the target account a fee for the service
5. TransferWithFee - Same as Transfer but debits the "from" account and the "to" account for the service
6. BurnWithFee - Same as Burn but debits the account a fee for the service

All fee's debited are credited to the "service" account

## Building
```
cd program
cargo build-bpf
```
