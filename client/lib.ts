// const borsh = require("borsh");

import { serialize, deserialize, deserializeUnchecked } from 'borsh';
import { Buffer } from 'buffer';
import {
    Keypair,
    AccountMeta,
    Connection,
    LAMPORTS_PER_SOL,
    PublicKey,
    SystemProgram,
    Transaction,
    TransactionInstruction,
    sendAndConfirmTransaction,
} from '@solana/web3.js';
import { Ok, Err, Result } from 'ts-results';

// Flexible class that takes properties and imbues them
// to the object instance
class Assignable {
    constructor(properties) {
        Object.keys(properties).map((key) => {
            return (this[key] = properties[key]);
        });
    }
}

// Our instruction payload vocabulary
class Payload extends Assignable { }

// Borsh needs a schema describing the payload
const payloadSchema = new Map([
    [
        Payload,
        {
            kind: "struct",
            fields: [
                ["id", "u8"],
                ["key", "string"],
                ["value", "string"]
            ]
        }
    ]
]);

export class AccoundData extends Assignable { }

const dataSchema = new Map([
    [
        AccoundData,
        {
            kind: "struct",
            fields: [
                ["initialized", "u8"],
                ["tree_length", "u32"],
                ["map", { kind: 'map', key: 'string', value: 'string' }]
            ]
        }
    ]
]);

/**
 * Fetch program account data
 * @param {Connection} connection - Solana RPC connection
 * @param {PublicKey} account - Account public key to fetch data state from
 * @return {Promise<AccoundData>} - Deserialized Account Data
 */
export async function getAccountData(
    connection: Connection,
    account: PublicKey): Promise<Result<AccoundData, Error>> {
    let nameAccount = await connection.getAccountInfo(
        account,
        'processed'
    );
    return Ok(deserializeUnchecked(dataSchema, AccoundData, nameAccount.data))
}

// Instruction variant indexes
enum InstructionVariant {
    InitializeAccount = 0,
    MintKeypair,
    TransferKeypair,
    BurnKeypair,
}

/**
 *
 * @param {Connection} connection - Solana RPC connection
 * @param {PublicKey} programKey - Sample Program public key
 * @param {Keypair} signer - Wallet for signing and payment
 * @param {Buffer} instructionData - Serialized instruction buffer
 * @param {AccountMeta[]} accounts - Array of write/read accounts
 * @returns {Promise<string>} result - Transaction signature string
 */
async function submitTxn(
    connection: Connection,
    programKey: PublicKey,
    signer: Keypair,
    instructionData: Buffer,
    accounts: AccountMeta[]): Promise<Result<string, Error>> {
    // Create Solana Instruction
    const instruction = new TransactionInstruction({
        data: instructionData,
        keys: accounts,
        // keys: [
        //     { pubkey: account, isSigner: false, isWritable: true },
        //     { pubkey: wallet.publicKey, isSigner: false, isWritable: false },
        // ],
        programId: programKey
    });

    // Send Solana Transaction
    try {
        const transactionSignature = await sendAndConfirmTransaction(
            connection,
            new Transaction().add(instruction),
            [signer],
            {
                commitment: 'singleGossip',
                preflightCommitment: 'singleGossip',
            },
        );
        return Ok(transactionSignature)
    }
    catch (e) {
        return Err(new Error("Transaction Failed"))
    }
}

/**
 * Mint a key value pair to account
 * @param {Connection} connection - Solana RPC connection
 * @param {PublicKey} progId - Sample Program public key
 * @param {PublicKey} account - Target program owned account for Mint
 * @param {Keypair} wallet - Wallet for signing and payment
 * @param {string} mintKey - The key being minted key
 * @param {string} mintValue - The value being minted
 * @return {Promise<Keypair>} - Keypair
 */
export async function mintKV(
    connection: Connection,
    progId: PublicKey,
    account: PublicKey,
    signer: Keypair,
    mintKey: string,
    mintValue: string): Promise<Result<string, Error>> {

    // Construct the payload
    const mint = new Payload({
        id: InstructionVariant.MintKeypair,
        key: mintKey,
        value: mintValue
    });

    // Serialize the payload
    const mintSerBuf = Buffer.from(serialize(payloadSchema, mint));
    if (mintSerBuf.byteLength === 0) {
        return Err(new Error('Failed to serialize payload to buffer'))
    }

    let signature = await submitTxn(connection, progId, signer, mintSerBuf, [
        { pubkey: account, isSigner: false, isWritable: true },
        { pubkey: signer.publicKey, isSigner: false, isWritable: false },
    ])
    return signature;
}
/**
 *
 * @param {Connection} connection - Solana RPC connection
 * @param {PublicKey} progId - Sample Program public key
 * @param {PublicKey} from_account - Current owner of the key/value mint
 * @param {PublicKey} to_account - Target program owned account for key value pair
 * @param {Keypair} wallet - Wallet for signing and payment
 * @param {string} transferKey - The key being transfered
 * @return {Promise<Keypair>} - Keypair
 * @returns
 */
export async function transferKV(
    connection: Connection,
    progId: PublicKey,
    from_account: PublicKey,
    to_account: PublicKey,
    signer: Keypair,
    transferKey: string): Promise<Result<string, Error>> {

    // Construct the payload
    const mint = new Payload({
        id: InstructionVariant.TransferKeypair,
        key: transferKey,
        value: ''
    });

    // Serialize the payload
    const mintSerBuf = Buffer.from(serialize(payloadSchema, mint));
    if (mintSerBuf.byteLength === 0) {
        return Err(new Error('Failed to serialize payload to buffer'))
    }

    let signature = await submitTxn(connection, progId, signer, mintSerBuf, [
        { pubkey: from_account, isSigner: false, isWritable: true },
        { pubkey: to_account, isSigner: false, isWritable: true },
        { pubkey: signer.publicKey, isSigner: false, isWritable: false },
    ])
    return signature;
}

/**
 * Burn a key from account
 * @param {Connection} connection - Solana RPC connection
 * @param {PublicKey} progId - Sample Program public key
 * @param {PublicKey} account - Target program owned account for Mint
 * @param {Keypair} wallet - Wallet for signing and payment
 * @param {string} mintKey - The key being minted key
 * @param {string} mintValue - The value being minted
 * @return {Promise<Keypair>} - Keypair
 */
export async function burnK(
    connection: Connection,
    progId: PublicKey,
    account: PublicKey,
    wallet: Keypair,
    mintKey: string): Promise<Result<string, Error>> {

    // Construct the payload
    const mint = new Payload({
        id: InstructionVariant.BurnKeypair,
        key: mintKey,
        value: ''
    });

    // Serialize the payload
    const mintSerBuf = Buffer.from(serialize(payloadSchema, mint));
    if (mintSerBuf.byteLength === 0) {
        return Err(new Error('Failed to serialize payload to buffer'))
    }

    let signature = await submitTxn(connection, progId, wallet, mintSerBuf, [
        { pubkey: account, isSigner: false, isWritable: true },
        { pubkey: wallet.publicKey, isSigner: false, isWritable: false },
    ])
    return signature;
}
