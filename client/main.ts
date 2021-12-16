#!/usr/bin/env node

import {
    mint_keys_setup,
    getProgramKeys,
    transfer_keys_setup,
    burn_keys_setup,
    try_to_pubkey
} from './keyfile';
import {
    AccoundData,
    burnK,
    getAccountData,
    mintKV,
    transferKV
} from "./lib"
import { Keypair, PublicKey, Connection, clusterApiUrl } from '@solana/web3.js';

/**
 * Entry point for script
 */
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
            let wallstring: string = null
            if ((argv.base === 'user1' || argv.base === 'user2') && argv.wallet == null) {
                wallstring = argv.base
            }
            else if (argv.wallet) {
                wallstring = argv.wallet
            }
            else {
                console.log("Need '-w' or '--wallet' argument specified")
                return
            }
            const result = await mint_keys_setup(argv.base, wallstring)
            if (result.ok) {
                let [userAccount, userWallet] = result.val
                let programKeys = await getProgramKeys(process.cwd())
                if (programKeys.ok) {
                    const connection = new Connection(clusterApiUrl(argv.url), "confirmed");
                    let result = await mintKV(
                        connection,
                        programKeys.val.publicKey,
                        userAccount,
                        userWallet,
                        argv.key,
                        argv.value)
                }
                else
                    console.log(programKeys.val)
            }
            else
                console.log(result.val)
        })

        // Transfer
        .command('transfer', 'Transfer a key value pair from one account to another', (yargs) => {
            return yargs
                .option('t', {
                    alias: 'to',
                    demandOption: true,
                    describe: 'Address to transfer key value to',
                })
        }, async (argv) => {
            let wallstring: string = null
            if ((argv.base === 'user1' || argv.base === 'user2') && argv.wallet == null) {
                wallstring = argv.base
            }
            else if (argv.wallet) {
                wallstring = argv.wallet
            }
            else {
                console.log("Need '-w' or '--wallet' argument specified")
                return
            }
            const result = await transfer_keys_setup(argv.base, argv.to, wallstring)
            if (result.ok) {
                let [userFromAccount, userToAccount, userWallet] = result.val
                let programKeys = await getProgramKeys(process.cwd())
                if (programKeys.ok) {
                    const connection = new Connection(clusterApiUrl(argv.url), "confirmed");
                    let result = await transferKV(
                        connection,
                        programKeys.val.publicKey,
                        userFromAccount,
                        userToAccount,
                        userWallet,
                        argv.key)
                }
                else
                    console.log(programKeys.val)
            }
            else
                console.log(result.val)
        })

        // Burn
        .command('burn', 'Burn a key value pair from an account', () => {
        }, async (argv) => {
            let wallstring: string = null
            if ((argv.base === 'user1' || argv.base === 'user2') && argv.wallet == null) {
                wallstring = argv.base
            }
            else if (argv.wallet) {
                wallstring = argv.wallet
            }
            else {
                console.log("Need '-w' or '--wallet' argument specified")
                return
            }
            const result = await burn_keys_setup(argv.base, wallstring)
            if (result.ok) {
                let [userAccount, userWallet] = result.val
                let programKeys = await getProgramKeys(process.cwd())
                if (programKeys.ok) {
                    const connection = new Connection(clusterApiUrl(argv.url), "confirmed");
                    let result = await burnK(
                        connection,
                        programKeys.val.publicKey,
                        userAccount,
                        userWallet,
                        argv.key)
                }
                else
                    console.log(programKeys.val)
            }
            else
                console.log(result.val)
        })
        .option('b', {
            alias: 'base',
            demandOption: true,
            global: true,
            describe: "Required account 'mint', 'transfer' or 'burn'. Can be Base58 account string or " +
                'keyfile path or ' +
                'user1 or user2 (from sample keys in repo)',
        })
        .option('w', {
            alias: 'wallet',
            demandOption: false,
            global: true,
            describe: "If not specifying 'user1' or 'user2' as 'base' or 'to' options this is required " +
                'Can be keyfile path or ' +
                'user1 or user2 (from sample keys in repo)',
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
        .option('k', {
            alias: 'key',
            global: true,
            demandOption: true,
            describe: 'Key of key value pair used by command',
            type: 'string',
        })
        .argv;
}

entry()