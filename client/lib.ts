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
 * @param {Keypair} wallet - Wallet for signing and payment
 * @return {Promise<AccoundData>} - Keypair
 */
export async function getAccountData(connection: Connection, account: Keypair): Promise<AccoundData> {
    let nameAccount = await connection.getAccountInfo(
        account.publicKey,
        'processed'
    );
    return deserializeUnchecked(dataSchema, AccoundData, nameAccount.data)
}

// Instruction variant indexes
enum InstructionVariant {
    InitializeAccount = 0,
    MintKeypair,
    TransferKeypair,
    BurnKeypair,
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
    wallet: Keypair,
    mintKey: string,
    mintValue: string): Promise<string> {

    // Construct the payload
    const mint = new Payload({
        id: InstructionVariant.MintKeypair,
        key: mintKey,
        value: mintValue

    });

    // Serialize the payload
    const mintSerBuf = Buffer.from(serialize(payloadSchema, mint));

    // Create Solana Instruction
    const instruction = new TransactionInstruction({
        data: mintSerBuf,
        keys: [
            { pubkey: account, isSigner: false, isWritable: true },
            { pubkey: wallet.publicKey, isSigner: false, isWritable: false },
        ],
        programId: progId
    });

    // Send Solana Transaction
    const transactionSignature = await sendAndConfirmTransaction(
        connection,
        new Transaction().add(instruction),
        [wallet],
        {
            commitment: 'singleGossip',
            preflightCommitment: 'singleGossip',
        },
    );
    console.log("Signature = ", transactionSignature)
    return transactionSignature;
}
