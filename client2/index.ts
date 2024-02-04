import {
    Keypair,
    Connection,
    Transaction,
    sendAndConfirmTransaction,
    PublicKey,
    SystemProgram
} from '@solana/web3.js';
import * as fs from 'fs';

import dotenv from 'dotenv';

dotenv.config();

// Read the configuration from config.json
const configPath = 'config.json';
const config = JSON.parse(fs.readFileSync(configPath, 'utf-8'));

// Configure the connection to the Solana devnet
const connection = new Connection(config.rpc_url, 'confirmed');

// Your Solana wallet's private key
// const privateKey = fs.readFileSync(config.keypair_path, 'utf-8').trim();
const secretKeyBytes = JSON.parse(fs.readFileSync(config.keypair_path, 'utf-8'));
const fromWallet = Keypair.fromSecretKey(Uint8Array.from(secretKeyBytes));

const programId = new PublicKey(config.program_id);

//TODO read from .env
const payer = Keypair.fromSecretKey(Buffer.from(process.env.PRIVATE_KEY || '', 'hex'));
const tokenAccount = new PublicKey(config.token_account_id);
const dexProgramId = new PublicKey(config.dex_program_id);

// Function to send a transaction to the Solana program
async function sendTransaction(): Promise<void> {

    // TODO - read from config
    const toWallet = Keypair.generate();
    // const toWalletPublicKey = new PublicKey('YOUR_PREDEFINED_PUBLIC_KEY');

    // the amount to transfer, in lamports
    const amount = 100;

    // Create a new transaction
    const transaction = new Transaction().add(
        // Your program's instruction

        SystemProgram.transfer({
            fromPubkey: fromWallet.publicKey,
            toPubkey: toWallet.publicKey,
            lamports: amount,
        })
    );

    // Sign the transaction
    transaction.feePayer = fromWallet.publicKey;
    transaction.recentBlockhash = (
        await connection.getRecentBlockhash()
    ).blockhash;
    transaction.sign(fromWallet);

    // Send the transaction
    const signature = await sendAndConfirmTransaction(connection, transaction, [
        fromWallet,
    ]);

    console.log('Transaction confirmed:', signature);
}

// Call the sendTransaction function
sendTransaction().catch(console.error);

// TODO
async function swapTokensForSol() {
  const swapTransaction = new Transaction();

  swapTransaction.sign(payer);
  const swapSignature = await connection.sendTransaction(swapTransaction);

  const swapConfirmedTransaction = await connection.confirmTransaction(swapSignature);
  console.log('Swap transaction confirmed:', swapConfirmedTransaction);
}

// TODO
async function manualSwap(amount: number, devPercentage: number, marketingPercentage: number) {
  const manualSwapTransaction = new Transaction();

  // Sign and send the manualSwap transaction
  manualSwapTransaction.sign(payer);
  const manualSwapSignature = await connection.sendTransaction(manualSwapTransaction);

  // Wait for the manualSwap transaction to be confirmed
  const manualSwapConfirmedTransaction = await connection.confirmTransaction(manualSwapSignature);
  console.log('ManualSwap transaction confirmed:', manualSwapConfirmedTransaction);
}