// web/components/solana/transaction-utils.ts

import { Connection, PublicKey, Transaction, TransactionInstruction, sendAndConfirmTransaction, Signer, Keypair } from '@solana/web3.js';
import { WalletContextState } from '@solana/wallet-adapter-react';
import { Program, Provider, web3 } from '@project-serum/anchor';


export async function createVault(
    // mintPubkey: PublicKey,
    mintKeyPair: Keypair,
    // ownerPubkey: PublicKey,
    ownerKeypair: Keypair,
    programId: PublicKey,
    wallet: WalletContextState,
    connection: Connection
  ) {
    const transaction = new Transaction().add(
      new TransactionInstruction({
        keys: [
          { pubkey: mintKeyPair.publicKey, isSigner: true, isWritable: true },
          { pubkey: ownerKeypair.publicKey, isSigner: true, isWritable: true },
          { pubkey: new PublicKey('SysvarRent111111111111111111111111111111111'), isSigner: false, isWritable: false },
        //   { pubkey: new PublicKey('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA'), isSigner: false, isWritable: false }
        ],
        programId: programId,
        data: Buffer.from([0]) // The instruction data
      })
    );
  
    if (!wallet.publicKey) {
      throw new Error('Wallet not connected');
    }
  
    // Get a recent blockhash from the connection
    const { blockhash } = await connection.getRecentBlockhash();
    transaction.recentBlockhash = blockhash;
    transaction.feePayer = wallet.publicKey;
  
    const ownerBalance = await connection.getBalance(ownerKeypair.publicKey);
    const walletBalance = await connection.getBalance(wallet.publicKey);

    console.log(`Owner balance: ${ownerBalance}`);
    console.log(`Wallet balance: ${walletBalance}`);

    // Sign the transaction with the owner keypair
    // transaction.partialSign(ownerKeypair);
    try {
        // Sign the transaction with the wallet
        const signedTransaction = await wallet.signTransaction!(transaction);
        console.log("Signed transaction details:", signedTransaction);

        // const signature = await sendAndConfirmTransaction(
        //     connection,
        //     signedTransaction,
        //     [], // Including the owner keypair as a signer
        //     { skipPreflight: false, commitment: 'singleGossip' }
        // );
        // Send and confirm the transaction with the necessary signers

        const signature = await wallet.sendTransaction(transaction, connection, { skipPreflight: false, preflightCommitment: 'singleGossip' });

        // const signature = await sendAndConfirmTransaction(
        //     connection,
        //     signedTransaction,
        //     [ownerKeypair, mintKeyPair],
        //     // [],
        //     // [wallet.publicKey], // No additional signers needed
        //     { skipPreflight: true, commitment: 'singleGossip' }
        // );

        // If no signature is returned, throw an error
        if (!signature) {
            throw new Error('Transaction did not return a signature');
        }

        console.log('Transaction successful with signature:', signature);
        return signature;
    } catch (err) {
        const error = err as Error;
        console.error('Transaction failed', error);
        console.error('Error details:', {
            message: error.message,
            stack: error.stack,
            transaction: JSON.stringify(transaction),
        });

        if (error.message.includes('Transaction cancelled')) {
            console.error('Possible cause: User rejected the transaction or it was cancelled.');
        }

        throw error; // Re-throw the error after logging
    }
  }
