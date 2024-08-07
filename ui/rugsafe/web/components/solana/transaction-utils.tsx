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
    const vaultKeypair = Keypair.generate(); // Generate keypair for vault account

    const mintPubkey = mintKeypair.publicKey;
    const ownerPubkey = wallet.publicKey as PublicKey;
    const vaultPubkey = vaultKeypair.publicKey;
    
    console.log("mintPubkey:", mintPubkey.toString());
    
    // return;
    const transaction = new Transaction().add(
        new TransactionInstruction({
            keys: [
                { pubkey: wallet.publicKey as PublicKey, isSigner: true, isWritable: true },
                { pubkey: mintPubkey, isSigner: true, isWritable: true },
                { pubkey: vaultPubkey, isSigner: true, isWritable: true }, // Add vault account
                { pubkey: rent, isSigner: false, isWritable: true },
                { pubkey: spl, isSigner: false, isWritable: true },
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
            signers: [mintKeypair, vaultKeypair]
        });
    console.log('Transaction successful with signature:', signature);
    return signature;

}

export async function deposit(
    programId: PublicKey,
    mintPubkey: PublicKey,
    vaultPubkey: PublicKey,
    userTokenAPubkey: PublicKey,
    userATokenAPubkey: PublicKey,
    depositAmount: number,
    wallet: WalletContextState,
    connection: Connection
) {
    const rentPubkey = new PublicKey("SysvarRent111111111111111111111111111111111");
    const splPubkey = TOKEN_PROGRAM_ID;

    // Prepare deposit instruction data
    let depositInstructionData = Buffer.alloc(9);
    depositInstructionData.writeUInt8(1, 0); // Instruction ID for "Deposit"
    depositInstructionData.writeBigUInt64LE(BigInt(depositAmount), 1);

    const depositInstruction = new TransactionInstruction({
        programId,
        keys: [
            { pubkey: wallet.publicKey as PublicKey, isSigner: true, isWritable: true }, // Payer
            { pubkey: mintPubkey, isSigner: false, isWritable: true }, // Mint account
            { pubkey: vaultPubkey, isSigner: false, isWritable: true }, // Vault account
            { pubkey: userTokenAPubkey, isSigner: true, isWritable: true }, // User's Token account
            { pubkey: userATokenAPubkey, isSigner: true, isWritable: true }, // User's aToken account
            { pubkey: rentPubkey, isSigner: false, isWritable: false }, // Rent sysvar
            { pubkey: splPubkey, isSigner: false, isWritable: false }, // SPL Token Program
            { pubkey: SystemProgram.programId, isSigner: false, isWritable: false }, // System Program
        ],
        data: depositInstructionData,
    });

    const transaction = new Transaction().add(depositInstruction);

    try {
        const { blockhash } = await connection.getRecentBlockhash();
        transaction.recentBlockhash = blockhash;
        transaction.feePayer = wallet.publicKey as PublicKey;

        // Sign the transaction with wallet
        const signature = await wallet.sendTransaction(transaction, connection, {
            skipPreflight: false,
            preflightCommitment: 'confirmed',
        });

        // Confirm the transaction
        await connection.confirmTransaction(signature, 'confirmed');
        console.log('Transaction successful with signature:', signature);
        return signature;
    } catch (error) {
        console.error('Transaction failed', error);
        throw error;
    }
}
