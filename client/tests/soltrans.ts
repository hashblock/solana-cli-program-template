
import {
    burn_keys_setup,
    getProgramKeys,
    mint_keys_setup,
    transfer_keys_setup
} from '../keyfile';
import { Connection, clusterApiUrl } from '@solana/web3.js';
import {
    burnK,
    getAccountData,
    mintKV,
    transferKV
} from "../lib"
import { assert } from 'chai';
import { describe } from 'mocha';

describe("Chain testing", async () => {
    it('test_mint_devnet_pass', async () => {
        let mint_keys_op = await mint_keys_setup('user1', 'user1')
        assert(mint_keys_op.ok)
        const [user1Account, user1Wallet] = mint_keys_op.unwrap()
        let progpair_op = await getProgramKeys(process.cwd())
        assert(progpair_op.ok)
        const progpair = progpair_op.unwrap()
        const connection = new Connection(clusterApiUrl("devnet"), "confirmed");
        const result = await mintKV(
            connection,
            progpair.publicKey,
            user1Account,
            user1Wallet,
            "ts key2",
            "ts first value2")
        assert(result.ok)
        const signature = result.unwrap();
        assert(signature.length > 0)
        const deser_result = await getAccountData(connection, user1Account)
        assert(deser_result.ok)
        const account = deser_result.val
        assert(account["map"].size > 0)
        assert(account["map"].get("ts key2") === "ts first value2")


    })
    it('test_mint_devnet_fail', async () => {
        let mint_keys_op = await mint_keys_setup('user1', 'user1')
        assert(mint_keys_op.ok)
        const [user1Account, user1Wallet] = mint_keys_op.unwrap()
        let progpair_op = await getProgramKeys(process.cwd())
        assert(progpair_op.ok)
        const progpair = progpair_op.unwrap()
        const connection = new Connection(clusterApiUrl("devnet"), "confirmed");
        let result = await mintKV(
            connection,
            progpair.publicKey,
            user1Account,
            user1Wallet,
            "ts key2",
            "ts first value2")
        assert(result.err)
    })
    it('test_transfer_devnet_pass', async () => {
        let transfer_keys_op = await transfer_keys_setup('user1', 'user2', 'user1')
        assert(transfer_keys_op.ok)
        const [user1Account, user2Account, user1Wallet] = transfer_keys_op.unwrap()
        let progpair_op = await getProgramKeys(process.cwd())
        assert(progpair_op.ok)
        const progpair = progpair_op.unwrap()
        const connection = new Connection(clusterApiUrl("devnet"), "confirmed");
        let result = await transferKV(
            connection,
            progpair.publicKey,
            user1Account,
            user2Account,
            user1Wallet,
            "ts key2")
        assert(result.ok)
        assert(result.unwrap().length > 0)
        const deser_result = await getAccountData(connection, user2Account)
        assert(deser_result.ok)
        const account = deser_result.val
        assert(account["map"].size > 0)
        assert(account["map"].get("ts key2") === "ts first value2")

    })
    it('test_burn_devnet_pass', async () => {
        let burn_keys_op = await burn_keys_setup('user2', 'user2')
        assert(burn_keys_op.ok)
        const [user2Account, user1Wallet] = burn_keys_op.unwrap()
        let progpair_op = await getProgramKeys(process.cwd())
        assert(progpair_op.ok)
        const progpair = progpair_op.unwrap()
        const connection = new Connection(clusterApiUrl("devnet"), "confirmed");
        let result = await burnK(
            connection,
            progpair.publicKey,
            user2Account,
            user1Wallet,
            "ts key2")
        assert(result.ok)
        assert(result.unwrap().length > 0)
        const deser_result = await getAccountData(connection, user2Account)
        assert(deser_result.ok)
        const account = deser_result.val
        // devnet may have other k/v pairs
        assert(account["map"].size >= 0)
        assert(account["map"].has("ts key2") === false)
    })
});