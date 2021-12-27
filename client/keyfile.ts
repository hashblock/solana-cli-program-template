import Base58 from "base-58";
import { Keypair, PublicKey } from '@solana/web3.js';
import fs from 'fs';
import { Ok, Err, Result } from 'ts-results';

export const progPath = "/keys/program/SampGgdt3wioaoMZhC6LTSbg4pnuvQnSfJpDYeuXQBv.json";
export const user1AccountPath = "/keys/accounts/user1_account.json";
export const user1AccountWallet = "/keys/accounts/user1_wallet.json";
export const user2AccountPath = "/keys/accounts/user2_account.json";
export const user2AccountWallet = "/keys/accounts/user2_wallet.json";

/**
 * Load (read) a file
 * @param {string} path - to file
 * @returns {Promise<string>} - Json string
 */
export async function get_file_content(path: string): Promise<Result<string, Error>> {
    return Ok(await fs.promises.readFile(path, 'utf8'))
}

/**
 * Load (read) a Keypair from file
 * @param {string} path - to file
 * @returns {Promise<Result<Keypair, Error>>} - Keypair or Error
 */
export async function get_as_keys(path: string): Promise<Result<Keypair, Error>> {
    const secretKeyString = (await get_file_content(path));
    if (secretKeyString.ok) {
        return Ok(Keypair.fromSecretKey(Uint8Array.from(JSON.parse(secretKeyString.val))))
    }
}

/**
 * Load (read) a PublicKey from Keypair from file
 * @param {string} path - to file
 * @return {Promise<Result<PublicKey, Error>>} - PublicKey or Error
 */
export async function get_as_pubkey(path: string): Promise<Result<PublicKey, Error>> {
    const secretKeyString = await get_as_keys(path);
    if (secretKeyString.ok)
        return Ok(secretKeyString.val.publicKey)
}



/**
 * Load programs Keypair from file.
 * @param {string} rootDir - base path .
 * @return {Promise<Keypair>} - Keypair
 */
export async function getProgramKeys(rootDir: string): Promise<Result<Keypair, Error>> {
    return await get_as_keys(rootDir + progPath)
}

/**
 * Load user1 Keypair from file.
 * @param {string} rootDir - base path
 * @return {Promise<Keypair>} - Keypair
 */
export async function getUser1Keys(rootDir: string): Promise<Result<Keypair, Error>> {
    return await get_as_keys(rootDir + user1AccountPath)
}

/**
 * Load user1 funding Wallet Keypair from file.
 * @param {string} rootDir - base path
 * @return {Promise<Keypair>} - Keypair
 */
export async function getUser1Wallet(rootDir: string): Promise<Result<Keypair, Error>> {
    return await get_as_keys(rootDir + user1AccountWallet)
}

/**
 * Load user2 Keypair from file.
 * @param {string} rootDir - base path
 * @return {Promise<Keypair>} - Keypair
 */
export async function getUser2Keys(rootDir: string): Promise<Result<Keypair, Error>> {
    return await get_as_keys(rootDir + user2AccountPath)
}

/**
 * Load user2 funding Wallet Keypair from file.
 * @param {string} rootDir - base path
 * @return {Promise<Keypair>} - Keypair
 */
export async function getUser2Wallet(rootDir: string): Promise<Result<Keypair, Error>> {
    return await get_as_keys(rootDir + user2AccountWallet)
}

/**
 * Trys to get Public key from string or file
 * @param {string} arg - string
 * @return {Promise<PublicKey>} - PublicKey
 */
export async function try_to_pubkey(arg: string): Promise<Result<PublicKey, Error>> {
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
        const key = (await getUser1Keys(process.cwd()))
        if (key.ok)
            return Ok(key.val.publicKey)
    }
    else if (arg === 'user2') {
        const key = (await getUser2Keys(process.cwd()))
        if (key.ok)
            return Ok(key.val.publicKey)
    }
    else {
        if (Base58.encode(Base58.decode(arg)) === arg) {
            try {
                return Ok(new PublicKey(arg))
            }
            catch (e) {
                return Err(new Error("Unable to create key from: " + arg))
            }
        }
        else {
            console.log("Not a valid keyfile or base58 string")
            return Err(new Error("Not a valid keyfile or base58 string: " + arg))
        }
    }
}

/**
 * Trys to get Wallet from file or predefined
 * @param {string} arg - string
 * @return {Promise<Keypair>} - Keypair
 */
export async function try_to_wallet_keypair(arg: string): Promise<Result<Keypair, Error>> {
    if (arg.endsWith(".json")) {
        const kres = await get_as_keys(arg)
        if (kres.ok)
            return kres
    }
    else if (arg === 'user1') {
        return (await getUser1Wallet(process.cwd()))
    }
    else if (arg === 'user2') {
        return (await getUser2Wallet(process.cwd()))
    }
    else {
        console.log("Not a valid keyfile or predefined wallet (user1, user2)")
        return Err(new Error("Not a valid keyfile or predefined wallet (user1, user2)"))
    }
}

export type ValidMintOrBurnPair = [PublicKey, Keypair];
export type ValidTransferTriple = [PublicKey, PublicKey, Keypair];

export async function mint_keys_setup(
    for_account: string,
    with_wallet: string): Promise<Result<ValidMintOrBurnPair, Error>> {
    const key = await try_to_pubkey(for_account)
    const wallet = await try_to_wallet_keypair(with_wallet)
    if (key.ok && wallet.ok) {
        let outpair: ValidMintOrBurnPair = [key.val, wallet.val]
        return Ok(outpair)
    }
    else {
        return Err(new Error("Failed to get necessary keys for mint"))
    }
}

export async function burn_keys_setup(
    for_account: string,
    with_wallet: string): Promise<Result<ValidMintOrBurnPair, Error>> {
    const key = await try_to_pubkey(for_account)
    const wallet = await try_to_wallet_keypair(with_wallet)
    if (key.ok && wallet.ok) {
        let outpair: ValidMintOrBurnPair = [key.val, wallet.val]
        return Ok(outpair)
    }
    else {
        return Err(new Error("Failed to get necessary keys for burn"))
    }
}

export async function transfer_keys_setup(
    from_account: string,
    to_account: string,
    with_wallet: string): Promise<Result<ValidTransferTriple, Error>> {

    const from_key = await try_to_pubkey(from_account)
    const to_key = await try_to_pubkey(to_account)
    const wallet = await try_to_wallet_keypair(with_wallet)

    if (from_key.ok && to_key.ok && wallet.ok) {
        let outtrip: ValidTransferTriple = [from_key.val, to_key.val, wallet.val]
        return Ok(outtrip)
    }
    else {
        return Err(new Error("Failed to get necessary keys for burn"))
    }
}