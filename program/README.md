# Sample program
The sample program is intended to provide a fundemental frame for your own programs.

## General Anatomy of a Solana Program
Solana programs use, but are not required, a common modular approach by breaking
up the semantics of a program into separate source files and folders.

1. `program/entry_point.rs` - This is the entry point when an instruction is sent to a specific program (yours), typically delegating the 3 arguments to a main dispatching function in...
2. `program/processor.rs` - This is the main body of execution. If your program has more than one (1) instruction to implement, it typically
will leverage the `program/instruction.rs` to assist in the deserializing of the instruction data. At which point, based on resolving
what instruction is identified, it will invoke the appropriate function to fulfill the intended behavior. Note that deserializing instruction data is different from serializating/deserializing account data
3. `program/instruction.rs` - Commonly declares an instruction enum with 1 entry for each instruction of your program. You can declare the enum with tuple structs to simplify instruction decoding
4. `program/account_state.rs` or `program/state.rs` - Contains the implementation of serialization and desearlization of data in an accounts allocated data array. This file usually contains the structures that the functions in `program/processor.rs` operate on.
5. `program/error.rs` - Contains the custom errors your program may assert if there is a problem with execution
6. `tests/lib.rs` - A good place to put unit testing for your program leveraging this `solana_program_test`. The `ProgramTest` structure will load your program, in a stripped down runtime context, and enable involing program instructions

You could technically combine all this into one source file but following the above will align with a good portion of Solana programs structures

## What About this Program?
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
## Unit Testing
```
cd program
cargo test-bpf -- --test-threads=1 --nocapture
```
