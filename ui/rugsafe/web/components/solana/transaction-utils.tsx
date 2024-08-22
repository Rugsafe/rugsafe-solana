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


// Define Vault and VaultRegistry classes
class Vault {
    vaultAccount: PublicKey;
    mintAccount: PublicKey;
    owner: PublicKey;

    static LEN: number = 32 * 3;

    constructor(fields: { vaultAccount: Uint8Array; mintAccount: Uint8Array; owner: Uint8Array }) {
        this.vaultAccount = new PublicKey(fields.vaultAccount);
        this.mintAccount = new PublicKey(fields.mintAccount);
        this.owner = new PublicKey(fields.owner);
    }

    static deserialize(input: Uint8Array): Vault {
        return new Vault({
            vaultAccount: input.slice(0, 32),
            mintAccount: input.slice(32, 64),
            owner: input.slice(64, 96),
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
        let offset = 4;

        for (let i = 0; i < vaultCount; i++) {
            const vault = Vault.deserialize(data.slice(offset, offset + Vault.LEN));
            vaults.push(vault);
            offset += Vault.LEN;
        }

        return new VaultRegistry(vaults);
    }
}


Vault.LEN = 96; // 32 * 5 bytes for each Pubkey


const VaultSchema = new Map([
    [
        Vault,
        {
            kind: 'struct',
            fields: [
                ['vaultAccount', [32]],
                ['mintAccount', [32]],
                ['owner', [32]],
            ],
        },
    ],
]);

const VaultRegistrySchema = new Map([
    [
        VaultRegistry,
        {
            kind: 'struct',
            fields: [
                ['vaults', [Vault]],
            ],
        },
    ],
]);

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
        new PublicKey("DG3jdET19heUQjp8fdL54FBvFd5oFWZZjCG8XgmFAHQJ"), //mint
        wallet.publicKey as PublicKey,
        true,
        TOKEN_PROGRAM_ID,
        ASSOCIATED_TOKEN_PROGRAM_ID
    )

    // Derive the associated token account for ATokenA
    const atokenaPubkey = await getAssociatedTokenAddress(
        mintATokenAPubkey,
        wallet.publicKey,
        false,
        TOKEN_PROGRAM_ID,
        ASSOCIATED_TOKEN_PROGRAM_ID
    );

    const [pda, _bump] = await PublicKey.findProgramAddress([Buffer.from('vault_registry')], programId);

    const createVaultInstructionData = Buffer.from([0]);

    const transaction = new Transaction().add(
        new TransactionInstruction({
            keys: [
                { pubkey: wallet.publicKey, isSigner: true, isWritable: true },
                { pubkey: mintTokenAPubkey, isSigner: true, isWritable: true },
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
        signers: [mintTokenAKeypair, mintATokenAKeypair]
    });

    console.log('Transaction successful with signature:', signature);
    return signature;
}

export async function deposit(
    programId: PublicKey,
    mintPubkey: PublicKey,
    vaultPubkey: PublicKey,
    userTokenAPubkey: PublicKey,
    // userATokenAPubkey: Keypair,
    userATokenAPubkey: PublicKey,
    depositAmount: number,
    wallet: WalletContextState,
    connection: Connection
) {

    const rentPubkey = new PublicKey("SysvarRent111111111111111111111111111111111");
    const splPubkey = TOKEN_PROGRAM_ID;

    console.log("programId", programId.toString())
    console.log("mintPubkey", mintPubkey.toString())
    console.log("vaultPubkey", vaultPubkey.toString())
    console.log("userTokenAPubkey", userTokenAPubkey.toString())
    console.log("userATokenAPubkey", userATokenAPubkey.toString())
    console.log("depositAmount", depositAmount)
    
    // Prepare deposit instruction data
    // let depositInstructionData = Buffer.alloc(32);
    // depositInstructionData.writeUInt8(1, 0); // Instruction ID for "Deposit"
    // depositInstructionData.writeBigUInt64LE(BigInt(depositAmount), 1);

    

    // console.log("depositInstructionData", depositInstructionData)
    const [stateAccountPDA, bump] = await PublicKey.findProgramAddress([Buffer.from('vault_registry')], programId);
    console.log("stateAccountPDA", stateAccountPDA)
    console.log("bump", bump)
    
    // const amount = 1;
    // const data = Buffer.from([1, ...new Uint8Array(new BN(amount).toArray('le', 8))]);
    const amount = 10;
    const data = Buffer.from([1, ...new Uint8Array(new BN(amount).toArray('le', 8))]);


    const depositInstruction = new TransactionInstruction({
        programId,
        keys: [
            { pubkey: wallet.publicKey as PublicKey, isSigner: true, isWritable: true }, // Payer
            { pubkey: mintPubkey, isSigner: false, isWritable: true }, // Mint account
            { pubkey: vaultPubkey, isSigner: false, isWritable: true }, // Vault account
            { pubkey: userTokenAPubkey, isSigner: false, isWritable: true }, // User's Token account
            // { pubkey: userATokenAPubkey.publicKey, isSigner: false, isWritable: true }, // User's aToken account
            { pubkey: userATokenAPubkey, isSigner: false, isWritable: true }, // User's aToken account
            { pubkey: rentPubkey, isSigner: false, isWritable: true }, // Rent sysvar
            { pubkey: splPubkey, isSigner: false, isWritable: true }, // SPL Token Program
            { pubkey: SystemProgram.programId, isSigner: false, isWritable: true }, // System Program
            { pubkey: stateAccountPDA, isSigner: false, isWritable: true }, 

        ],
        // data: depositInstructionData,
        // data: Buffer.from([1, 100]),
        data,
    });

    console.log("depositInstruction")
    console.log(depositInstruction)

    const transaction = new Transaction().add(depositInstruction);
    console.log(transaction)
    try {
        const { blockhash } = await connection.getRecentBlockhash();
        transaction.recentBlockhash = blockhash;
        transaction.feePayer = wallet.publicKey as PublicKey;

        // Sign the transaction with wallet
        // const signature = await wallet.sendTransaction(transaction, connection, {
        //     skipPreflight: false,
        //     preflightCommitment: 'confirmed',
        // });

        const signature = await wallet.sendTransaction(
            transaction, 
            connection, 
            { 
                skipPreflight: true, 
                preflightCommitment: 'singleGossip', 
                // signers: [mintKeypair, userTokenAccount]
                // signers: [wallet, mintKeypair, userTokenAccount]
                // signers: [userATokenAPubkey]
                signers: []

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
    const userTokenKeypair = Keypair.generate();
    const userTokenAccount = userTokenKeypair.publicKey;


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
                { pubkey: userTokenAccount, isSigner: true, isWritable: true }, // PDA, not signed
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
                signers: [userTokenKeypair]

            });

        return signature;
    } catch (error) {
        console.error("Error in callFaucet:", error);
        throw error;
    }
};


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