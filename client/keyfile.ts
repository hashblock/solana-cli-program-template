import Base58 from "base-58";
import { Keypair, PublicKey } from '@solana/web3.js';
import fs from 'fs';

export const progPath = "/keys/program/SampGgdt3wioaoMZhC6LTSbg4pnuvQnSfJpDYeuXQBv.json";
export const user1AccountPath = "/keys/accounts/user1_account.json";
export const user1AccountWallet = "/keys/accounts/user1_wallet.json";
export const user2AccountPath = "/keys/accounts/user2_account.json";
export const user2AccountWallet = "/keys/accounts/user2_wallet.json";

/**
 * Load (read) a file
 * @param path
 * @returns {Promise<string>} - string
 */
export async function get_file_content(path: string): Promise<string> {
    return await fs.promises.readFile(path, 'utf8')
}

export async function get_as_keys(path: string): Promise<Keypair> {
    const secretKeyString = await get_file_content(path);
    return Keypair.fromSecretKey(Uint8Array.from(JSON.parse(secretKeyString)))
}

/**
 * Returns load (read) file as PublicKey
 * @param path
 * @return {Promise<PublicKey>} - PublicKey
 */
export async function get_as_pubkey(path: string): Promise<PublicKey> {
    const secretKeyString = await get_as_keys(path);
    return secretKeyString.publicKey
}



/**
 * Load programs Keypair from file.
 * @param {string} rootDir - base path .
 * @return {Promise<Keypair>} - Keypair
 */
export async function getProgramKeys(rootDir: string): Promise<Keypair> {
    return await get_as_keys(rootDir + progPath)
}

/**
 * Load user1 Keypair from file.
 * @param {string} rootDir - base path
 * @return {Promise<Keypair>} - Keypair
 */
export async function getUser1Keys(rootDir: string): Promise<Keypair> {
    return await get_as_keys(rootDir + user1AccountPath)
}

/**
 * Load user1 funding Wallet Keypair from file.
 * @param {string} rootDir - base path
 * @return {Promise<Keypair>} - Keypair
 */
export async function getUser1Wallet(rootDir: string): Promise<Keypair> {
    return await get_as_keys(rootDir + user1AccountWallet)
}

/**
 * Load user2 Keypair from file.
 * @param {string} rootDir - base path
 * @return {Promise<Keypair>} - Keypair
 */
export async function getUser2Keys(rootDir: string): Promise<Keypair> {
    return await get_as_keys(rootDir + user2AccountPath)
}

/**
 * Load user2 funding Wallet Keypair from file.
 * @param {string} rootDir - base path
 * @return {Promise<Keypair>} - Keypair
 */
export async function getUser2Wallet(rootDir: string): Promise<Keypair> {
    return await get_as_keys(rootDir + user2AccountWallet)
}

/**
 * Trys to get Public key from string or file
 * @param {string} arg - string
 * @return {Promise<PublicKey>} - PublicKey
 */
export async function try_to_pubkey(arg: string): Promise<PublicKey> {
    if (arg.endsWith(".json")) {
        await get_as_pubkey(arg)
            .then(result => {
                return result
            })
            .catch(err => {
                console.log("Unable to load key from: ", arg)
                return null
            })
    }
    else if (arg === 'user1') {
        return (await getUser1Keys(process.cwd())).publicKey
    }
    else if (arg === 'user2') {
        return (await getUser2Keys(process.cwd())).publicKey
    }
    else {
        if (Base58.encode(Base58.decode(arg)) === arg) {
            try {
                return new PublicKey(arg)
            }
            catch (e) {
                console.log("Unable to create key from: ", arg)
                return null
            }
        }
        else {
            console.log("Not a valid keyfile or base58 string")
            return null
        }
    }
}