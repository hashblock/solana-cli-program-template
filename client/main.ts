#!/usr/bin/env node

import { try_to_pubkey } from './keyfile';
import {
    AccoundData,
    getAccountData,
    mintKV
} from "./lib.js"
import { Keypair, PublicKey, Connection, clusterApiUrl } from '@solana/web3.js';

async function entry() {
    const yargs = require('yargs/yargs')(process.argv.slice(2));
    const argv = yargs
        .usage('Usage: $0 <command> [options]')

        // Mint
        .command('mint', 'Mint a key value pair to an account', (yargs) => {
            return yargs
                .option('v', {
                    alias: 'value',
                    demandOption: true,
                    describe: 'Value of key value pair used by command',
                })
        }, async (argv) => {
            const mintKey = await try_to_pubkey(argv.base)
            if (mintKey) {
                console.log('doing mint ', argv.url, ' key:', argv.key, ' value:', argv.value, ' to:', argv.base)
            }
        })

        // Transfer
        .command('transfer', 'Transfer a key value pair from one account to another', (yargs) => {
            return yargs
                .option('t', {
                    alias: 'to',
                    demandOption: true,
                    describe: 'Address to transfer key value to',
                })
        }, (argv) => {
            console.log("transfer ", argv.url, ' key:', argv.key, ' from:', argv.base, ' to:', argv.to)
        })

        // Burn
        .command('burn', 'Burn a key value pair from an account', () => {
        }, (argv) => {
            console.log('burn ', argv.url, ' key:', argv.key, ' from:', argv.base)
        })
        .option('b', {
            alias: 'base',
            demandOption: true,
            global: true,
            describe: 'Can be Base58 account string or ' +
                'keyfile path or ' +
                'user1 or user2 (from sample keys in repo)',
        })
        .option('u', {
            alias: 'url',
            global: true,
            demandOption: true,
            describe: 'Specify Solana RPC url',
            choices: ['localnet', 'devnet'],
            type: 'string',
        })
        .option('k', {
            alias: 'key',
            global: true,
            demandOption: true,
            describe: 'Key of key value pair used by command',
            type: 'string',
        })
        .argv;
}

// describe('Sample Program', async () => {
//     const progpair: Keypair = await getProgramKeys(process.cwd())
//     const user1Account: Keypair = await getUser1Keys(process.cwd())
//     const user1Wallet: Keypair = await getUser1Wallet(process.cwd())

//     const connection = new Connection(clusterApiUrl("devnet"), "confirmed");
//     const tsKey = "ts key"
//     const tsValue = "ts first value"

// let result = await mintKV(
//     connection,
//     progpair.publicKey,
//     user1Account.publicKey,
//     user1Wallet,
//     "ts key",
//     "ts first value")

// console.log(result)

// let u1data: AccoundData = await getAccountData(connection, user1Account)
// expect(u1data['initialized']).equal(1)
// expect(u1data['tree_length']).equal(32)
// expect(u1data["map"].size).equal(1)
// expect(u1data["map"].get(tsKey)).equal(tsValue)
// console.log(u1data)
// });

entry()