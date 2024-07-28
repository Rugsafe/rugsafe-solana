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
    

    ////////////////////////creating accoiunt 
    const rent_ca = await connection.getMinimumBalanceForRentExemption(MintLayout.span); // Mint account size is 82 bytes

    console.log("MintLayout.span", MintLayout.span)
    const createMintAccountIx = SystemProgram.createAccount({
        fromPubkey: wallet.publicKey as PublicKey,
        newAccountPubkey: mintPubkey,
        lamports: rent_ca,
        space: MintLayout.span, //82,
        programId: TOKEN_PROGRAM_ID,  // SPL Token program owns the mint account
    });

    console.log("Mint account instruction created:", createMintAccountIx);


    const initMintIx = createInitializeMintInstruction(
        mintPubkey,  // mint account public key
        0,  // decimals
        ownerPubkey,  // mint authority
        ownerPubkey,  // freeze authority (optional, can be set to ownerPubkey if you don't want to specify)
        spl  // program ID of the SPL Token program
    );

    console.log("init mint:", initMintIx);


    const transaction_ca = new Transaction()
        .add(createMintAccountIx)
        // .add(initMintIx);

    transaction_ca.feePayer = ownerPubkey;

    console.log(transaction_ca)

    // Simulate the transaction
    const simulation = await connection.simulateTransaction(transaction_ca);
    console.log("Simulation result:", simulation);
    try {
        const signature_ca = await wallet.sendTransaction(transaction_ca, connection, { 
            skipPreflight: true,  //false throws unexpected
            preflightCommitment: 'singleGossip', 
        });
        console.log('Transaction successful with signature:', signature_ca);
        await connection.confirmTransaction(signature_ca, 'confirmed');
    } catch (error: any) {
        console.error('Transaction failed', error);
        alert('Transaction failed: ' + error.message);
    }
    // await connection.confirmTransaction(signature_ca, 'confirmed');
    ////////////////// end  creating accounts


    return;
    // const transaction = new Transaction().add(
    //     new TransactionInstruction({
    //         keys: [
    //             { pubkey: wallet.publicKey as PublicKey,    isSigner: true,  isWritable: true },
    //             // { pubkey: wallet.publicKey as PublicKey,    isSigner: false, isWritable: true },
    //             { pubkey: mintPubkey as PublicKey,          isSigner: false, isWritable: true },
    //             { pubkey: ownerPubkey,                      isSigner: false, isWritable: true },
    //             { pubkey: rent,                             isSigner: false, isWritable: false },
    //             { pubkey: spl,                              isSigner: false, isWritable: true },

    //         ],
    //         programId: programId,
    //         data: Buffer.from([0]), // The instruction data
    //     })
    // );


    // console.log("transaction msg", transaction);
    // if (!wallet.publicKey) {
    //     throw new Error('Wallet not connected');
    // }

    // // Get a recent blockhash from the connection
    // const { blockhash } = await connection.getRecentBlockhash();
    // transaction.recentBlockhash = blockhash;
    // transaction.feePayer = wallet.publicKey;

    // const ownerBalance = await connection.getBalance(ownerPubkey);
    // const mintBalance = await connection.getBalance(mintPubkey);
    // const walletBalance = await connection.getBalance(wallet.publicKey);

    // console.log(`Owner balance: ${ownerBalance}`);
    // console.log(`Wallet balance: ${walletBalance}`);
    // console.log(`Mint balance: ${mintBalance}`);

    // // Sign and send the transaction using wallet adapter
    // // try {
    // const signature = await wallet.sendTransaction(transaction, 
    //     connection, 
    //     { 
    //         skipPreflight: true, 
    //         preflightCommitment: 'singleGossip', 
    //         // signers: [wallet.publicKey]
    //     });
    // console.log('Transaction successful with signature:', signature);
    // return signature;



    // } catch (error) {
        // console.error('Transaction failed', error);
        // throw error;
    // }
}
