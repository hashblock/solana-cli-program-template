import 'mocha';
import { expect } from 'chai';

import { Keypair, PublicKey } from '@solana/web3.js';
import BufferLayout from 'buffer-layout';
import { loadKeypair, progpath } from './keyfile';
describe('Sample Program', async () => {
    const progfile = process.cwd() + progpath
    const progpair: Keypair = await loadKeypair(progfile)
    console.log(progpair.publicKey)
});
