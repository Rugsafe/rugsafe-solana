import React, { useState, useEffect } from 'react';
import { PublicKey, Connection } from '@solana/web3.js';
import { useWallet } from '@solana/wallet-adapter-react';
import { deposit } from './solana/transaction-utils'; // Adjust the import path accordingly
import { getAssociatedTokenAddress, TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID} from '@solana/spl-token';
const LOCALHOST_URL = 'http://127.0.0.1:8899';
const CONTRACT_PROGRAM_ID = 'AVFEXtCiwxuBHuMUsnFGoFB44ymVAbMn3QsN6f6pw5yA';

const ListVaults = () => {
    const [vaults, setVaults] = useState<Array<{pubkey: string, mint: string, userTokenAccount: string, userATokenAccount: string}>>([]);
    const wallet = useWallet();
    const connection = new Connection(LOCALHOST_URL, 'confirmed');

    useEffect(() => {
        if (wallet.connected) {
            listVaults();
        }
    }, [wallet.connected]);

    const listVaults = async () => {
        try {
            const ownerPublicKey = wallet.publicKey as PublicKey;
            const accounts = await connection.getTokenAccountsByOwner(ownerPublicKey, {
                programId: TOKEN_PROGRAM_ID,
            });

            const relevantVaults = accounts.value.map(account => {
                const data = account.account.data;
                if (isVaultAccount(data)) {
                    // Assuming that the vault data can provide or map to necessary addresses
                    const mint = getMintFromData(data); // Replace with actual logic
                    const userTokenAccount = getUserTokenAccountFromData(data); // Replace with actual logic
                    const userATokenAccount = getUserATokenAccountFromData(data); // Replace with actual logic

                    return {
                        pubkey: account.pubkey.toBase58(),
                        mint,
                        userTokenAccount,
                        userATokenAccount,
                    };
                }
                return null;
            }).filter(Boolean);

            setVaults(relevantVaults as any);
        } catch (error) {
            console.error('Error listing vaults:', error);
        }
    };

    const isVaultAccount = (data: Buffer): boolean => {
        // Implement your logic to identify vault accounts
        // For example, based on specific data structure or identifier
        return true; // Placeholder - replace with your own logic
    };

    const getMintFromData = (data: Buffer): string => {
        // Extract the mint public key from the data
        return 'YOUR_MINT_PUBKEY'; // Replace with actual logic to extract this
    };

    const getUserTokenAccountFromData = (data: Buffer): string => {
        // Extract the user's token account public key from the data
        return 'USER_TOKEN_ACCOUNT'; // Replace with actual logic to extract this
    };

    const getUserATokenAccountFromData = (data: Buffer): string => {
        // Extract the user's aToken account public key from the data
        return 'USER_ATOKEN_ACCOUNT'; // Replace with actual logic to extract this
    };

    const handleDeposit = async (vault: {
        mintTokenAAccount: string, 
        mintATokenAAccount: string, 
        owner: string, 
        vaultAccount: string}) => {
        try {
            console.log("vault")
            console.log(vault)

    
            const programId = new PublicKey(CONTRACT_PROGRAM_ID);
            const mintTokenAPubkey = new PublicKey(vault.mintTokenAAccount);
            const mintATokenAPubkey = new PublicKey(vault.mintATokenAAccount);
            
            // const vaultPubkey = new PublicKey("nBzomwsoJpu8CiRL5f7iJkN5cLJryMeTwPC8nNJciqr");
            const vaultPubkey = await getAssociatedTokenAddress(
                mintTokenAPubkey,           // Mint address
                programId,        // Owner of the associated token account
                false,                   // Allow owner off curve
                TOKEN_PROGRAM_ID,        // Token program ID
                ASSOCIATED_TOKEN_PROGRAM_ID // Associated token program ID
            );
            
            console.log(`vaultPubkey: ${vaultPubkey}`)
            
            // const userTokenAPubkey = new PublicKey("Dof5p3fEhZhXttrPeEPiKwLoac5ftRyJJnma24ZYF4qZ");
            const userTokenAPubkey = await getAssociatedTokenAddress(
                mintTokenAPubkey,           // Mint address
                wallet.publicKey as PublicKey,        // Owner of the associated token account
                false,                   // Allow owner off curve
                TOKEN_PROGRAM_ID,        // Token program ID
                ASSOCIATED_TOKEN_PROGRAM_ID // Associated token program ID
            );
            
            console.log(`userTokenAPubkey: ${userTokenAPubkey}`)

            // const userATokenAPubkey = new PublicKey(vault.mint);
            const userATokenAPubkey = await getAssociatedTokenAddress(
                mintATokenAPubkey,           // Mint address
                wallet.publicKey as PublicKey,        // Owner of the associated token account
                false,                   // Allow owner off curve
                TOKEN_PROGRAM_ID,        // Token program ID
                ASSOCIATED_TOKEN_PROGRAM_ID // Associated token program ID
            );

            console.log(`userATokenAPubkey: ${userATokenAPubkey}`)

            const depositAmount = 100; // Example deposit amount
    
            const signature = await deposit(
                programId,
                mintTokenAPubkey,
                mintATokenAPubkey,
                vaultPubkey,
                userTokenAPubkey,
                userATokenAPubkey,
                depositAmount,
                wallet,
                connection
            );
    
            console.log('Deposit transaction signature:', signature);
        } catch (error) {
            console.error('Deposit failed:', error);
        }
    };
    
    
    

    const handleWithdraw = (vaultAddress: string) => {
        console.log(`Withdraw clicked for vault: ${vaultAddress}`);
        // Implement withdraw logic here
    };

    return (
        <div>
            <div className="vault-cards">
                {vaults.map((vault, index) => (
                    <div key={index} className="vault-card" style={{border: '1px solid #ccc', padding: '20px', marginBottom: '10px', borderRadius: '5px'}}>
                        <h3>Vault #{index + 1}</h3>
                        <p>Address: {vault.pubkey}</p>
                        <p>Mint: {vault.mint}</p>
                        <p>User Token Account: {vault.userTokenAccount}</p>
                        <p>User aToken Account: {vault.userATokenAccount}</p>
                        <div className="button-group" style={{display: 'flex', gap: '10px'}}>
                            <button className="btn" onClick={() => handleDeposit(vault)}>
                                Deposit
                            </button>
                            <button className="btn" onClick={() => handleWithdraw(vault.pubkey)}>
                                Withdraw
                            </button>
                        </div>
                    </div>
                ))}
            </div>
        </div>
    );
};

export default ListVaults;