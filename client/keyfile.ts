import { Keypair } from '@solana/web3.js';
import fs from 'fs';

export const progpath = "/keys/program/SampGgdt3wioaoMZhC6LTSbg4pnuvQnSfJpDYeuXQBv.json";
export const user1path = "/keys/accounts/user1_account.json";
export const user2path = "/keys/accounts/user2_account.json";

async function get_content(path: string) {
    // const content: Promise<string> = promisify(fs.readFile)(path, { encoding: "UTF-8" })
    return await fs.promises.readFile(path, 'utf8');
}
/**
 * Load a Keypair from file.
 * @param {string} path - path to keypair file.
 * @return {Promise<Keypair>} - Keypair
 */
export async function loadKeypair(path: string): Promise<Keypair> {
    console.log(path)
    const secretKeyString = await get_content(path);
    const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
    return Keypair.fromSecretKey(secretKey);
}