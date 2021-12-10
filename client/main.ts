import 'mocha';
import { expect } from 'chai';

import { Keypair, PublicKey, Connection, clusterApiUrl } from '@solana/web3.js';
import { getProgramKeys, getUser1Keys, getUser1Wallet } from './keyfile';
import {
    mintKV
} from "./lib.js"

describe('Sample Program', async () => {
    const progpair: Keypair = await getProgramKeys(process.cwd())
    const user1Account: Keypair = await getUser1Keys(process.cwd())
    const user1Wallet: Keypair = await getUser1Wallet(process.cwd())

    const connection = new Connection(clusterApiUrl("devnet"), "confirmed");

    let result = await mintKV(
        connection,
        progpair.publicKey,
        user1Account.publicKey,
        user1Wallet,
        "ts key",
        "ts first value")

    console.log(result)

});
