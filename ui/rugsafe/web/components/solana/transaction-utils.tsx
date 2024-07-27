import { Connection, PublicKey, Transaction, TransactionInstruction, sendAndConfirmTransaction } from '@solana/web3.js';
import { WalletContextState } from '@solana/wallet-adapter-react';

export async function createVault(
    mintPubkey: PublicKey,
    ownerPubkey: PublicKey,
    programId: PublicKey,
    wallet: WalletContextState,
    connection: Connection
) {
    console.log("programId", programId);

    const rent = new PublicKey("11111111111111111111111111111111");
    const spl = new PublicKey('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA')
    const transaction = new Transaction().add(
        new TransactionInstruction({
            keys: [
                // { pubkey: mintPubkey, isSigner: false, isWritable: true },
                { pubkey: spl, isSigner: false, isWritable: true },
                { pubkey: ownerPubkey, isSigner: false, isWritable: true },
                { pubkey: rent, isSigner: false, isWritable: false },
            ],
            programId: programId,
            data: Buffer.from([0]), // The instruction data
        })
    );

    console.log("transaction msg", transaction);
    if (!wallet.publicKey) {
        throw new Error('Wallet not connected');
    }

    // Get a recent blockhash from the connection
    const { blockhash } = await connection.getRecentBlockhash();
    transaction.recentBlockhash = blockhash;
    transaction.feePayer = wallet.publicKey;

    const ownerBalance = await connection.getBalance(ownerPubkey);
    const mintBalance = await connection.getBalance(mintPubkey);
    const walletBalance = await connection.getBalance(wallet.publicKey);

    console.log(`Owner balance: ${ownerBalance}`);
    console.log(`Wallet balance: ${walletBalance}`);
    console.log(`Mint balance: ${mintBalance}`);

    // Sign and send the transaction using wallet adapter
    try {
        const signature = await wallet.sendTransaction(transaction, connection, { skipPreflight: false, preflightCommitment: 'singleGossip' });
        console.log('Transaction successful with signature:', signature);
        return signature;
    } catch (error) {
        console.error('Transaction failed', error);
        throw error;
    }
}
