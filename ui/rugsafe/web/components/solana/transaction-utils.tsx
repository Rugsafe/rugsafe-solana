import { Connection, PublicKey, Transaction, TransactionInstruction, sendAndConfirmTransaction, Keypair, LAMPORTS_PER_SOL, SystemProgram } from '@solana/web3.js';
import { WalletContextState } from '@solana/wallet-adapter-react';
import { TOKEN_PROGRAM_ID, MintLayout, createInitializeMintInstruction } from '@solana/spl-token';

export async function createVault(
    programId: PublicKey,
    wallet: WalletContextState,
    connection: Connection
) {
    console.log("programId", programId);

    const rent = new PublicKey("SysvarRent111111111111111111111111111111111");
    const spl = new PublicKey('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA');
    (spl == TOKEN_PROGRAM_ID) ? console.log("t") : console.log('f');
    (spl.toString() == TOKEN_PROGRAM_ID.toString()) ? console.log("t") : console.log('f');
    console.log("SPL Token Program ID:", spl.toString());
    console.log("TOKEN_PROGRAM_ID:", TOKEN_PROGRAM_ID.toString());

    const mintKeypair = Keypair.generate();
    const mintPubkey = mintKeypair.publicKey;
    // const mintPubkey = new PublicKey("46Uxi9EoxSnC72w6xGjArfHcDVLBKUQFR9ydEigs1oXp");
    const ownerPubkey = wallet.publicKey as PublicKey;

    console.log("mintPubkey:", mintPubkey.toString());
    // console.log("ownerPubkey:", ownerPubkey.toString());
    
    // return;
    const transaction = new Transaction().add(
        new TransactionInstruction({
            keys: [
                { pubkey: wallet.publicKey as PublicKey, isSigner: true, isWritable: true },
                { pubkey: mintPubkey, isSigner: true, isWritable: true },
                // { pubkey: ownerPubkey, isSigner: false, isWritable: true },
                { pubkey: rent, isSigner: false, isWritable: false },
                { pubkey: spl, isSigner: false, isWritable: false },
                { pubkey: SystemProgram.programId, isSigner: false, isWritable: true }

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
    // try {
    const signature = await wallet.sendTransaction(transaction, 
        connection, 
        { 
            skipPreflight: true, 
            preflightCommitment: 'singleGossip', 
            // signers: [wallet.publicKey]
            signers: [mintKeypair]
        });
    console.log('Transaction successful with signature:', signature);
    return signature;

}
