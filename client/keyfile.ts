import { Keypair } from '@solana/web3.js';
import fs from 'fs';

export const progPath = "/keys/program/SampGgdt3wioaoMZhC6LTSbg4pnuvQnSfJpDYeuXQBv.json";
export const user1AccountPath = "/keys/accounts/user1_account.json";
export const user1AccountWallet = "/keys/accounts/user1_wallet.json";
export const user2AccountPath = "/keys/accounts/user2_account.json";
export const user2AccountWallet = "/keys/accounts/user2_wallet.json";

async function get_content(path: string) {
    // const content: Promise<string> = promisify(fs.readFile)(path, { encoding: "UTF-8" })
    return await fs.promises.readFile(path, 'utf8');
}

/**
 * Load programs Keypair from file.
 * @param {string} rootDir - base path .
 * @return {Promise<Keypair>} - Keypair
 */
export async function getProgramKeys(rootDir: string): Promise<Keypair> {
    const secretKeyString = await get_content(rootDir + progPath);
    const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
    return Keypair.fromSecretKey(secretKey);
}

/**
 * Load user1 Keypair from file.
 * @param {string} rootDir - base path
 * @return {Promise<Keypair>} - Keypair
 */
export async function getUser1Keys(rootDir: string): Promise<Keypair> {
    const secretKeyString = await get_content(rootDir + user1AccountPath);
    const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
    return Keypair.fromSecretKey(secretKey);
}

/**
 * Load user1 funding Wallet Keypair from file.
 * @param {string} rootDir - base path
 * @return {Promise<Keypair>} - Keypair
 */
export async function getUser1Wallet(rootDir: string): Promise<Keypair> {
    const secretKeyString = await get_content(rootDir + user1AccountWallet);
    const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
    return Keypair.fromSecretKey(secretKey);
}

/**
 * Load user2 Keypair from file.
 * @param {string} rootDir - base path
 * @return {Promise<Keypair>} - Keypair
 */
export async function getUser2Keys(rootDir: string): Promise<Keypair> {
    const secretKeyString = await get_content(rootDir + user2AccountPath);
    const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
    return Keypair.fromSecretKey(secretKey);
}

/**
 * Load user2 funding Wallet Keypair from file.
 * @param {string} rootDir - base path
 * @return {Promise<Keypair>} - Keypair
 */
export async function getUser2Wallet(rootDir: string): Promise<Keypair> {
    const secretKeyString = await get_content(rootDir + user2AccountWallet);
    const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
    return Keypair.fromSecretKey(secretKey);
}
