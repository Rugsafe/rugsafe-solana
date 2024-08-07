import React, { useState, useEffect } from 'react';
import { PublicKey, Connection } from '@solana/web3.js';
import { TOKEN_PROGRAM_ID } from '@solana/spl-token';
import { useWallet } from '@solana/wallet-adapter-react';
import { deposit } from './solana/transaction-utils'; // Adjust the import path accordingly

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

    const handleDeposit = async (vault: {pubkey: string, mint: string, userTokenAccount: string, userATokenAccount: string}) => {
        try {
            console.log('Vault Pubkey:', vault.pubkey);
            console.log('Mint Pubkey:', vault.mint);
            console.log('User Token Account Pubkey:', vault.userTokenAccount);
            console.log('User aToken Account Pubkey:', vault.userATokenAccount);
    
            const programId = new PublicKey(CONTRACT_PROGRAM_ID);
            const vaultPubkey = new PublicKey(vault.pubkey);
            const userTokenAPubkey = new PublicKey(vault.userTokenAccount);
            const userATokenAPubkey = new PublicKey(vault.userATokenAccount);
            const mintPubkey = new PublicKey(vault.mint);
            const depositAmount = 100; // Example deposit amount
    
            const signature = await deposit(
                programId,
                mintPubkey,
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
