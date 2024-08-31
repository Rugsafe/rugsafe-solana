import { 
    Connection, 
    PublicKey, 
    Transaction, 
    TransactionInstruction, 
    sendAndConfirmTransaction, 
    Keypair, 
    LAMPORTS_PER_SOL, 
    SystemProgram, 
    ParsedAccountData 
} from '@solana/web3.js';
import { WalletContextState } from '@solana/wallet-adapter-react';
import { getAssociatedTokenAddress,
        createAssociatedTokenAccountInstruction, 
        TOKEN_PROGRAM_ID, 
        ASSOCIATED_TOKEN_PROGRAM_ID, 
        MintLayout, 
        createInitializeMintInstruction 
} from '@solana/spl-token';
import BN from 'bn.js';






///////////////////////////////////////////////




class Vault {
    vaultAccount: PublicKey;
    mintTokenA: PublicKey;
    mintATokenA: PublicKey;
    owner: PublicKey;

    static LEN: number = 128; // 32 bytes * 4 Pubkeys

    constructor(fields: { vaultAccount: Uint8Array; mintTokenA: Uint8Array; mintATokenA: Uint8Array; owner: Uint8Array }) {
        this.vaultAccount = new PublicKey(fields.vaultAccount);
        this.mintTokenA = new PublicKey(fields.mintTokenA);
        this.mintATokenA = new PublicKey(fields.mintATokenA);
        this.owner = new PublicKey(fields.owner);
    }

    static deserialize(input: Uint8Array): Vault {
        // Logging each part of the slice to validate
        console.log("Raw input:", input);
        console.log("VaultAccount slice:", input.slice(0, 32));
        console.log("MintTokenA slice:", input.slice(32, 64));
        console.log("MintATokenA slice:", input.slice(64, 96));
        console.log("Owner slice:", input.slice(96, 128));
        
        return new Vault({
            vaultAccount: input.slice(0, 32),
            mintTokenA: input.slice(32, 64),
            mintATokenA: input.slice(64, 96),
            owner: input.slice(96, 128),
        });
    }
}

class VaultRegistry {
    vaults: Vault[];

    constructor(vaults: Vault[]) {
        this.vaults = vaults;
    }

    static deserialize(data: Uint8Array): VaultRegistry {
        const vaults: Vault[] = [];
        const vaultCount = new DataView(data.buffer).getUint32(0, true); // Read the vault count (4 bytes)
        let offset = 16; // Start after the count

        console.log(`Vault count: ${vaultCount}`);

        for (let i = 0; i < vaultCount; i++) {
            const vault = Vault.deserialize(data.slice(offset, offset + Vault.LEN));
            console.log(`Deserialized Vault #${i + 1}:`, vault);
            console.log(`mintATokenA for Vault #${i + 1}:`, vault.mintATokenA.toString());
            vaults.push(vault);
            offset += Vault.LEN;
        }

        return new VaultRegistry(vaults);
    }
}

Vault.LEN = 128; // 32 * 4 bytes for each Pubkey




////////////////////////////////////////////////////














// const VaultSchema = new Map([
//     [
//         Vault,
//         {
//             kind: 'struct',
//             fields: [
//                 ['vaultAccount', [32]],
//                 ['mintAccount', [32]],
//                 ['owner', [32]],
//             ],
//         },
//     ],
// ]);

// const VaultRegistrySchema = new Map([
//     [
//         VaultRegistry,
//         {
//             kind: 'struct',
//             fields: [
//                 ['vaults', [Vault]],
//             ],
//         },
//     ],
// ]);

export { Vault, VaultRegistry };
export async function createVault(
    programId: PublicKey,
    wallet: WalletContextState,
    connection: Connection
) {
    if (!wallet.publicKey) {
        throw new Error('Wallet not connected');
    }

    const rent = new PublicKey("SysvarRent111111111111111111111111111111111");
    const spl = TOKEN_PROGRAM_ID;

    // Generate keypair for Token A mint
    // const mintTokenAKeypair = Keypair.generate();
    // const mintTokenAPubkey = mintTokenAKeypair.publicKey;
    const mintTokenAPubkey = new PublicKey("DG3jdET19heUQjp8fdL54FBvFd5oFWZZjCG8XgmFAHQJ");

    // Generate keypair for AToken A mint
    const mintATokenAKeypair = Keypair.generate();
    const mintATokenAPubkey = mintATokenAKeypair.publicKey;

    // Derive the associated token account for the vault (Token A)
    // const vaultTokenAccount = await getAssociatedTokenAddress(
    //     mintTokenAPubkey,
    //     wallet.publicKey,
    //     true,
    //     TOKEN_PROGRAM_ID,
    //     ASSOCIATED_TOKEN_PROGRAM_ID
    // );

    const vaultTokenAccount = await getAssociatedTokenAddress(
        mintTokenAPubkey, //mint

        // note: here, we dont get a associated address for the wallet, but for the program
        // to decide what is the unique vault address that will hole tokens
        // wallet.publicKey as PublicKey,
        programId,
        
        
        true,
        TOKEN_PROGRAM_ID,
        ASSOCIATED_TOKEN_PROGRAM_ID
    )

    // Derive the associated token account for ATokenA
    // const atokenaPubkey = await getAssociatedTokenAddress(
    //     mintATokenAPubkey,
    //     wallet.publicKey,
    //     false,
    //     TOKEN_PROGRAM_ID,
    //     ASSOCIATED_TOKEN_PROGRAM_ID
    // );

    const [pda, _bump] = await PublicKey.findProgramAddress([Buffer.from('vault_registry')], programId);

    const createVaultInstructionData = Buffer.from([0]);

    const transaction = new Transaction().add(
        new TransactionInstruction({
            keys: [
                { pubkey: wallet.publicKey, isSigner: true, isWritable: true },
                { pubkey: mintTokenAPubkey, isSigner: false, isWritable: true },
                { pubkey: mintATokenAPubkey, isSigner: true, isWritable: true },
                // { pubkey: atokenaPubkey, isSigner: false, isWritable: true },
                { pubkey: vaultTokenAccount, isSigner: false, isWritable: true },
                { pubkey: rent, isSigner: false, isWritable: false },
                { pubkey: spl, isSigner: false, isWritable: false },
                { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
                { pubkey: pda, isSigner: false, isWritable: true },
                { pubkey: ASSOCIATED_TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
            ],
            programId: programId,
            data: createVaultInstructionData,
        })
    );

    const { blockhash } = await connection.getRecentBlockhash();
    transaction.recentBlockhash = blockhash;
    transaction.feePayer = wallet.publicKey;

    const signature = await wallet.sendTransaction(transaction, connection, { 
        skipPreflight: true, 
        preflightCommitment: 'processed', 
        // signers: [mintTokenAPubkey, mintATokenAKeypair]
        signers: [mintATokenAKeypair]
    });

    console.log('Transaction successful with signature:', signature);
    return signature;
}

// export async function deposit(
//     programId: PublicKey,
//     mintPubkey: PublicKey,
//     vaultPubkey: PublicKey,
//     userTokenAPubkey: PublicKey,
//     // userATokenAPubkey: Keypair,
//     userATokenAPubkey: PublicKey,
//     depositAmount: number,
//     wallet: WalletContextState,
//     connection: Connection
// ) {

//     const rentPubkey = new PublicKey("SysvarRent111111111111111111111111111111111");
//     const splPubkey = TOKEN_PROGRAM_ID;

//     console.log("programId", programId.toString())
//     console.log("mintPubkey", mintPubkey.toString())
//     console.log("vaultPubkey", vaultPubkey.toString())
//     console.log("userTokenAPubkey", userTokenAPubkey.toString())
//     console.log("userATokenAPubkey", userATokenAPubkey.toString())
//     console.log("depositAmount", depositAmount)
    
//     // Prepare deposit instruction data
//     // let depositInstructionData = Buffer.alloc(32);
//     // depositInstructionData.writeUInt8(1, 0); // Instruction ID for "Deposit"
//     // depositInstructionData.writeBigUInt64LE(BigInt(depositAmount), 1);

    

//     // console.log("depositInstructionData", depositInstructionData)
//     const [stateAccountPDA, bump] = await PublicKey.findProgramAddress([Buffer.from('vault_registry')], programId);
//     console.log("stateAccountPDA", stateAccountPDA)
//     console.log("bump", bump)
    
//     // const amount = 1;
//     // const data = Buffer.from([1, ...new Uint8Array(new BN(amount).toArray('le', 8))]);
//     const amount = 10;
//     const data = Buffer.from([1, ...new Uint8Array(new BN(amount).toArray('le', 8))]);


//     const depositInstruction = new TransactionInstruction({
//         programId,
//         keys: [
//             { pubkey: wallet.publicKey as PublicKey, isSigner: true, isWritable: true }, // Payer
//             { pubkey: mintPubkey, isSigner: false, isWritable: true }, // Mint account
//             { pubkey: vaultPubkey, isSigner: false, isWritable: true }, // Vault account
//             { pubkey: userTokenAPubkey, isSigner: false, isWritable: true }, // User's Token account
//             // { pubkey: userATokenAPubkey.publicKey, isSigner: false, isWritable: true }, // User's aToken account
//             { pubkey: userATokenAPubkey, isSigner: false, isWritable: true }, // User's aToken account
//             { pubkey: rentPubkey, isSigner: false, isWritable: true }, // Rent sysvar
//             { pubkey: splPubkey, isSigner: false, isWritable: true }, // SPL Token Program
//             { pubkey: SystemProgram.programId, isSigner: false, isWritable: true }, // System Program
//             { pubkey: stateAccountPDA, isSigner: false, isWritable: true }, 

//         ],
//         // data: depositInstructionData,
//         // data: Buffer.from([1, 100]),
//         data,
//     });

//     console.log("depositInstruction")
//     console.log(depositInstruction)

//     const transaction = new Transaction().add(depositInstruction);
//     console.log(transaction)
//     try {
//         const { blockhash } = await connection.getRecentBlockhash();
//         transaction.recentBlockhash = blockhash;
//         transaction.feePayer = wallet.publicKey as PublicKey;

//         // Sign the transaction with wallet
//         // const signature = await wallet.sendTransaction(transaction, connection, {
//         //     skipPreflight: false,
//         //     preflightCommitment: 'confirmed',
//         // });

//         const signature = await wallet.sendTransaction(
//             transaction, 
//             connection, 
//             { 
//                 skipPreflight: true, 
//                 preflightCommitment: 'singleGossip', 
//                 // signers: [mintKeypair, userTokenAccount]
//                 // signers: [wallet, mintKeypair, userTokenAccount]
//                 // signers: [userATokenAPubkey]
//                 signers: []

//             });

//         // Confirm the transaction
//         await connection.confirmTransaction(signature, 'confirmed');
//         console.log('Transaction successful with signature:', signature);
//         return signature;
//     } catch (error) {
//         console.error('Transaction failed', error);
//         throw error;
//     }
// }

export async function deposit(
    programId: PublicKey,
    mintTokenAPubkey: PublicKey,
    mintATokenAPubkey: PublicKey,
    // mintATokenAPubkey: Keypair,
    vaultPubkey: PublicKey,
    userTokenAPubkey: PublicKey,
    userATokenAPubkey: PublicKey,
    depositAmount: number,
    wallet: WalletContextState,
    connection: Connection
) {
    const rentPubkey = new PublicKey("SysvarRent111111111111111111111111111111111");
    const splPubkey = TOKEN_PROGRAM_ID;

    console.log("programId", programId.toString());
    console.log("mintTokenAPubkey", mintTokenAPubkey.toString());
    console.log("mintATokenAPubkey", mintATokenAPubkey.toString());
    console.log("vaultPubkey", vaultPubkey.toString());
    console.log("userTokenAPubkey", userTokenAPubkey.toString());
    console.log("userATokenAPubkey", userATokenAPubkey.toString());
    console.log("depositAmount", depositAmount);

    // Find the PDA for the state account
    const [stateAccountPDA, bump] = await PublicKey.findProgramAddress(
        [Buffer.from('vault_registry')],
        programId
    );
    console.log("stateAccountPDA", stateAccountPDA.toString());
    console.log("bump", bump);

    // Prepare the instruction data
    const data = Buffer.from([1, ...new Uint8Array(new BN(depositAmount).toArray('le', 8))]);

    const depositInstruction = new TransactionInstruction({
        programId,
        keys: [
            { pubkey: wallet.publicKey as PublicKey, isSigner: true, isWritable: true }, // Payer
            { pubkey: mintTokenAPubkey, isSigner: false, isWritable: true }, // TokenA Mint account
            { pubkey: mintATokenAPubkey, isSigner: false, isWritable: true }, // aTokenA Mint account
            // { pubkey: mintATokenAPubkey.publicKey, isSigner: false, isWritable: true }, // aTokenA Mint account
            { pubkey: vaultPubkey, isSigner: false, isWritable: true }, // Vault account
            { pubkey: userTokenAPubkey, isSigner: false, isWritable: true }, // User's TokenA account
            { pubkey: userATokenAPubkey, isSigner: false, isWritable: true }, // User's aTokenA account
            { pubkey: rentPubkey, isSigner: false, isWritable: false }, // Rent sysvar
            { pubkey: splPubkey, isSigner: false, isWritable: false }, // SPL Token Program
            { pubkey: SystemProgram.programId, isSigner: false, isWritable: false }, // System Program
            // { pubkey: stateAccountPDA, isSigner: false, isWritable: true }, // State account PDA
            { pubkey: ASSOCIATED_TOKEN_PROGRAM_ID, isSigner: false, isWritable: true }, // State account PDA
        ],
        data,
    });

    console.log("depositInstruction:", depositInstruction);

    const transaction = new Transaction().add(depositInstruction);
    console.log("Transaction:", transaction);

    try {
        const { blockhash } = await connection.getRecentBlockhash();
        transaction.recentBlockhash = blockhash;
        transaction.feePayer = wallet.publicKey as PublicKey;

        // Sign and send the transaction
        const signature = await wallet.sendTransaction(
            transaction,
            connection,
            {
                skipPreflight: true,
                preflightCommitment: 'singleGossip',
                signers: [] // No additional signers needed since wallet is signing
            }
        );

        // Confirm the transaction
        await connection.confirmTransaction(signature, 'confirmed');
        console.log('Transaction successful with signature:', signature);
        return signature;
    } catch (error) {
        console.error('Transaction failed:', error);
        throw error;
    }
}

export async function fetchVaultRegistry(stateAccountPubkey: PublicKey, connection: Connection) {
    try {
        // Fetch the account data from the blockchain
        const accountInfo = await connection.getAccountInfo(stateAccountPubkey);

        if (!accountInfo) {
            throw new Error('State account not found');
        }

        // Extract the data buffer
        const data = accountInfo.data;

        // Manually deserialize the data according to your VaultRegistry structure
        const vaultRegistry = VaultRegistry.deserialize(new Uint8Array(data));

        return vaultRegistry;
    } catch (error) {
        console.error('Failed to fetch or deserialize VaultRegistry:', error);
        throw error;
    }
}


export const callFaucet = async (
    programId: PublicKey,
    wallet: any,
    connection: Connection
) => {
    if (!wallet.publicKey) {
        throw new Error('Wallet not connected');
    }

    console.log("Program ID:", programId.toBase58());

    // Derive the mint PDA as you did in the test
    const [mintPublicKey] = await PublicKey.findProgramAddress(
        [Buffer.from("mint")],
        programId
    );

    const rent = new PublicKey("SysvarRent111111111111111111111111111111111");

    // Create user token account PDA
    // const [userTokenAccount] = await PublicKey.findProgramAddress(
    //     [
    //       wallet.publicKey.toBuffer(),
    //       TOKEN_PROGRAM_ID.toBuffer(),
    //       mintPublicKey.toBuffer(),
    //     ],
    //     TOKEN_PROGRAM_ID
    // );

    // v1
    // const userTokenKeypair = Keypair.generate();
    // const userTokenAccount = userTokenKeypair.publicKey;

    const userTokenAccount = await getAssociatedTokenAddress(
        mintPublicKey,           // Mint address
        wallet.publicKey,        // Owner of the associated token account
        false,                   // Allow owner off curve
        TOKEN_PROGRAM_ID,        // Token program ID
        ASSOCIATED_TOKEN_PROGRAM_ID // Associated token program ID
    );

        window.alert(userTokenAccount)
    console.log("Generated user token account:", userTokenAccount.toString());
    console.log("Mint Public Key:", mintPublicKey.toBase58());
    console.log("User Token Account:", userTokenAccount.toBase58());

    const amount = 1000;
    const data = Buffer.from([4, ...new Uint8Array(new BN(amount).toArray('le', 8))]);

    const transaction = new Transaction();

    // No need to add createAssociatedTokenAccountInstruction since you're using PDAs
    const userTokenAccountInfo = await connection.getAccountInfo(userTokenAccount);


    // Add the faucet instruction
    transaction.add(
        new TransactionInstruction({
            keys: [
                { pubkey: wallet.publicKey, isSigner: true, isWritable: true },
                { pubkey: userTokenAccount, isSigner: false, isWritable: true }, // PDA, not signed
                { pubkey: mintPublicKey, isSigner: false, isWritable: true },
                { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
                { pubkey: rent, isSigner: false, isWritable: false },
                { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
            ],
            programId,
            data: data,
        })
    );

    try {
        const { blockhash } = await connection.getRecentBlockhash();
        transaction.recentBlockhash = blockhash;
        transaction.feePayer = wallet.publicKey;

        const signature = await wallet.sendTransaction(
            transaction, 
            connection, 
            { 
                skipPreflight: true, 
                preflightCommitment: 'singleGossip', 
                // signers: [mintKeypair, userTokenAccount]
                // signers: [wallet, mintKeypair, userTokenAccount]
                // signers: [userTokenKeypair]
                signers: []

            });

        return signature;
    } catch (error) {
        console.error("Error in callFaucet:", error);
        throw error;
    }
};


// export const callFaucet = async (
//     programId: PublicKey,
//     wallet: any,
//     connection: Connection
// ) => {
//     if (!wallet.publicKey) {
//         throw new Error('Wallet not connected');
//     }

//     console.log("Program ID:", programId.toBase58());

//     // Derive the mint PDA
//     const [mintPublicKey] = await PublicKey.findProgramAddress(
//         [Buffer.from("mint")],
//         programId
//     );

//     const rent = new PublicKey("SysvarRent111111111111111111111111111111111");

//     // Derive the associated token account (ATA) for the user
//     const userTokenAccount = await PublicKey.createWithSeed(
//         wallet.publicKey,
//         "token_account",
//         TOKEN_PROGRAM_ID
//     );

//     console.log("Derived user token ATA:", userTokenAccount.toString());
//     console.log("Mint Public Key:", mintPublicKey.toBase58());

//     const amount = 1000;
//     const data = Buffer.from([4, ...new Uint8Array(new BN(amount).toArray('le', 8))]);

//     const transaction = new Transaction();

//     // Check if the ATA exists, if not, create it
//     const userTokenAccountInfo = await connection.getAccountInfo(userTokenAccount);

//     if (!userTokenAccountInfo) {
//         transaction.add(
//             createAssociatedTokenAccountInstruction(
//                 ASSOCIATED_TOKEN_PROGRAM_ID, // Associated token program ID
//                 TOKEN_PROGRAM_ID,            // Token program ID
//                 mintPublicKey,               // Mint address
//                 userTokenAccount,            // Associated token account address
//                 wallet.publicKey,            // Owner of the associated token account
//                 wallet.publicKey             // Payer of the transaction
//             )
//         );
//         console.log("ATA created:", userTokenAccount.toBase58());
//     } else {
//         console.log("ATA already exists:", userTokenAccount.toBase58());
//     }

//     // Add the faucet instruction
//     transaction.add(
//         new TransactionInstruction({
//             keys: [
//                 { pubkey: wallet.publicKey, isSigner: true, isWritable: true },
//                 { pubkey: userTokenAccount, isSigner: false, isWritable: true }, // ATA, not signed
//                 { pubkey: mintPublicKey, isSigner: false, isWritable: true },
//                 { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
//                 { pubkey: rent, isSigner: false, isWritable: false },
//                 { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
//             ],
//             programId,
//             data: data,
//         })
//     );

//     try {
//         const { blockhash } = await connection.getRecentBlockhash();
//         transaction.recentBlockhash = blockhash;
//         transaction.feePayer = wallet.publicKey;

//         const signature = await wallet.sendTransaction(
//             transaction, 
//             connection, 
//             { 
//                 skipPreflight: true, 
//                 preflightCommitment: 'singleGossip'
//             }
//         );

//         return signature;
//     } catch (error) {
//         console.error("Error in callFaucet:", error);
//         throw error;
//     }
// };








export const getTokenBalance = async (
    connection: Connection,
    wallet: any,
    mintPublicKey: PublicKey
) => {

    console.log("mintPublicKey: ", mintPublicKey.toBase58());
    const userTokenAccount2 = await getAssociatedTokenAddress(
        mintPublicKey,
        wallet.publicKey
    );
    console.log("userTokenAccount2: ", userTokenAccount2.toBase58());


    const userTokenAccount = new PublicKey("8r8vqPQAjG8MvL4uEgbLsD9ZYUHLSZp4GXHbtQ9MkY6Z")

    console.log("getTokenBalance, userTokenAccount: ", userTokenAccount.toBase58())
    const accountInfo = await connection.getParsedAccountInfo(userTokenAccount);

    if (accountInfo.value && 'parsed' in accountInfo.value.data) {
        const parsedData = accountInfo.value.data as ParsedAccountData;
        return parsedData.parsed.info.tokenAmount.uiAmount || 0;
    } else {
        return 0;
    }
};