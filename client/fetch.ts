#!/usr/bin/env node

import { try_to_pubkey } from './keyfile';
import { getAccountData, } from "./lib"
import { Connection, clusterApiUrl } from '@solana/web3.js';

/**
 * Entry point for script
 */
async function fetch_entry() {
    const yargs = require('yargs/yargs')(process.argv.slice(2));
    const argv = yargs
        .usage('Usage: $0 <command> [options]')
        .command('data', 'Fetch and deserialize account data', {
        }, async (argv) => {
            const result = await try_to_pubkey(argv.base)
            if (result.ok) {
                let userAccount = result.val
                const connection = new Connection(clusterApiUrl(argv.url), "confirmed");
                let data_result = await getAccountData(connection, userAccount)
                if (data_result.ok) {
                    const data = data_result.val
                    console.log("Deserialize data: ", data)
                }
                else
                    console.log(data_result)
            }
            else
                console.log(result)
        })
        .option('b', {
            alias: 'base',
            demandOption: true,
            global: true,
            describe: "Required account. Can be Base58 account string or " +
                'keyfile path or ' +
                'user1 or user2 (fetches from key/accounts sample keys in repo)',
        })
        .option('u', {
            alias: 'url',
            global: true,
            demandOption: true,
            describe: 'Specify Solana RPC url',
            choices: ['localnet', 'devnet'],
            default: 'devnet',
            type: 'string',
        })
        .argv;
}

fetch_entry()