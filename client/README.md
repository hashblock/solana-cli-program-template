## About
The 'client' is a Node/Typescript script that can be run from the command line.

## Node Setup

From the repo root folder install the node dependencies:

`npm install`

## Pre Running the Node Script

### devnet
The sample program the script interacts with is already deployed as are various account keys that are also on devnet. This is the default network but the [command line](#script-help) lets you change to `localnet`

### localnet
Refer to the building and running of the [sample program](../src/README.md)

## Running the Node Script

From the repo root folder:

### Build

`npm run build:client`

### Script help

Overall help

* `ts-node client/main.ts -h`

Command specific help
* `ts-node client/main.ts mint -h`
* `ts-node client/main.ts transfer -h`
* `ts-node client/main.ts burn -h`
* `ts-node client/fetch.ts data -h`

### Prebuilt Script Execution
#### Mint a new key/value pair
With Node: This runs on 'devnet' and mints key: 'newKey' and value: 'A new value' to: 'user1' from sample keys

`npm run do:MintUser1`

`npm run do:DataUser1`

#### Transfer a key/value pair
With Node: This runs on 'devnet' and transfers key: 'newKey' (and value) from: 'user1' to 'user2'

`npm run do:TransferUser1To2`

`npm run do:DataUser1`

`npm run do:DataUser2`

#### Burn a key/value pair
With Node: This runs on 'devnet' and burns key: 'newKey' from: 'user2'

`npm run do:BurnUser2`

`npm run do:DataUser1`

`npm run do:DataUser2`
