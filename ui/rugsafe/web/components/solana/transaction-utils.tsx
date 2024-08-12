import { Connection, PublicKey, Transaction, TransactionInstruction, sendAndConfirmTransaction, Keypair, LAMPORTS_PER_SOL, SystemProgram } from '@solana/web3.js';
import { WalletContextState } from '@solana/wallet-adapter-react';
import { TOKEN_PROGRAM_ID, MintLayout, createInitializeMintInstruction } from '@solana/spl-token';


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
    console.log("ownerPubkey", ownerPubkey)
    console.log("mintPubkey:", mintPubkey.toString());
    
    const [pda, bump] = await PublicKey.findProgramAddress([Buffer.from('vault_registry')], programId);
    console.log("pda", pda)
    console.log("bump", bump)
    // return;
    const transaction = new Transaction().add(
        new TransactionInstruction({
            keys: [
                { pubkey: wallet.publicKey as PublicKey, isSigner: true, isWritable: true },
                { pubkey: mintPubkey, isSigner: true, isWritable: true },
                { pubkey: vaultPubkey, isSigner: true, isWritable: true }, // Add vault account
                { pubkey: rent, isSigner: false, isWritable: true },
                { pubkey: spl, isSigner: false, isWritable: true },
                { pubkey: SystemProgram.programId, isSigner: false, isWritable: true },
                //pda
                // { pubkey: Keypair.generate().publicKey, isSigner: true, isWritable: true }, // state_account
                { pubkey: pda, isSigner: false, isWritable: true }, // Pass the PDA

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

    console.log(depositInstructionData)
    const [stateAccountPDA, bump] = await PublicKey.findProgramAddress([Buffer.from('vault_registry')], programId);
    console.log("stateAccountPDA", stateAccountPDA)
    console.log("bump", bump)
    
    const depositInstruction = new TransactionInstruction({
        programId,
        keys: [
            { pubkey: wallet.publicKey as PublicKey, isSigner: true, isWritable: true }, // Payer
            { pubkey: mintPubkey, isSigner: false, isWritable: true }, // Mint account
            { pubkey: vaultPubkey, isSigner: false, isWritable: true }, // Vault account
            { pubkey: userTokenAPubkey, isSigner: false, isWritable: true }, // User's Token account
            { pubkey: userATokenAPubkey, isSigner: false, isWritable: true }, // User's aToken account
            { pubkey: rentPubkey, isSigner: false, isWritable: true }, // Rent sysvar
            { pubkey: splPubkey, isSigner: false, isWritable: true }, // SPL Token Program
            { pubkey: SystemProgram.programId, isSigner: false, isWritable: true }, // System Program
            { pubkey: stateAccountPDA, isSigner: false, isWritable: true }, 

        ],
        data: depositInstructionData,
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