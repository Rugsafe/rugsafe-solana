import React, { useState, useEffect } from 'react';
import { PublicKey, Connection, Keypair } from '@solana/web3.js';
import { useWallet } from '@solana/wallet-adapter-react';
import { fetchVaultRegistry, deposit } from './solana/transaction-utils';

const LOCALHOST_URL = 'http://127.0.0.1:8899';
// const CONTRACT_PROGRAM_ID = 'AVFEXtCiwxuBHuMUsnFGoFB44ymVAbMn3QsN6f6pw5yA';
const CONTRACT_PROGRAM_ID = 'FobNvbQsK5BAniZC2oJhXakjcPiArpsthTGDnX9eHDVY';

const deriveStateAccountPDA = async (programId: PublicKey) => {
    const [stateAccountPDA, _] = await PublicKey.findProgramAddress(
        [Buffer.from('vault_registry')],
        programId
    );
    return stateAccountPDA;
};

const ListVaultsFromRegistry = () => {
    const [vaults, setVaults] = useState([]);
    const [balances, setBalances] = useState({});
    const wallet = useWallet();
    const connection = new Connection(LOCALHOST_URL, 'confirmed');

    useEffect(() => {
        if (wallet.connected) {
            listVaults();
        }
    }, [wallet.connected]);

    const listVaults = async () => {
        try {
            const programId = new PublicKey(CONTRACT_PROGRAM_ID);
            const stateAccountPubkey = await deriveStateAccountPDA(programId);
            const vaultRegistry = await fetchVaultRegistry(stateAccountPubkey, connection);
    
            const formattedVaults = vaultRegistry.vaults.map((vault) => ({
                vaultAccount: vault.vaultAccount.toBase58(),
                mintAccount: vault.mintAccount.toBase58(),
                owner: vault.owner.toBase58(),
            }));
    
            setVaults(formattedVaults);
            await fetchBalances(formattedVaults);
        } catch (error) {
            console.error('Error fetching vault registry:', error);
        }
    };

    const fetchBalances = async (vaults: any) => {
        const newBalances = {};
    
        for (const vault of vaults) {
            try {
                console.log(`Fetching balances for vault ${vault.vaultAccount}`);
                // console.log(`User Token Account: ${vault.userTokenAccount}`);
                // console.log(`User aToken Account: ${vault.userATokenAccount}`);
                console.log(`Vault Account: ${vault.vaultAccount}`);
    
                // const userTokenAccountBalance = await connection.getTokenAccountBalance(new PublicKey(vault.userTokenAccount));
                // const userATokenAccountBalance = await connection.getTokenAccountBalance(new PublicKey(vault.userATokenAccount));
                // const vaultTokenAccountBalance = await connection.getTokenAccountBalance(new PublicKey(vault.vaultAccount));
    
                // // console.log(`User Token Balance: ${userTokenAccountBalance.value.amount}`);
                // // console.log(`User aToken Balance: ${userATokenAccountBalance.value.amount}`);
                // console.log(`Vault Token Balance: ${vaultTokenAccountBalance.value.amount}`);
    
                // newBalances[vault.vaultAccount] = {
                //     userTokenBalance: userTokenAccountBalance.value.amount,
                //     userATokenBalance: userATokenAccountBalance.value.amount,
                //     vaultTokenBalance: vaultTokenAccountBalance.value.amount,
                // };
            } catch (error) {
                console.error(`Error fetching balances for vault ${vault.vaultAccount}:`, error);
            }
        }
    
        setBalances(newBalances);
    };

    const handleDeposit = async (vault: any) => {
        try {
            const programId = new PublicKey(CONTRACT_PROGRAM_ID);
            const vaultPubkey = new PublicKey(vault.vaultAccount);
            const mintPubkey = new PublicKey(vault.mintAccount);
            const depositAmount = 100;

             // Generate key pairs for the user's token and aToken accounts
            const userTokenAPubkey = Keypair.generate().publicKey;
            const userATokenAPubkey = Keypair.generate().publicKey;
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
            await fetchBalances(vaults); // Refresh balances after deposit
        } catch (error) {
            console.error('Deposit failed:', error);
        }
    };

    const handleWithdraw = (vaultAddress: any) => {
        console.log(`Withdraw clicked for vault: ${vaultAddress}`);
        // Implement withdraw logic here
    };

    return (
        <div>
            <div className="vault-cards">
                {vaults.map((vault, index) => (
                    <div key={index} className="vault-card" style={{ border: '1px solid #ccc', padding: '20px', marginBottom: '10px', borderRadius: '5px' }}>
                        <h3>Vault #{index + 1}</h3>
                        <p>Vault Account: {vault.vaultAccount}</p>
                        <p>Mint Account: {vault.mintAccount}</p>
                        {/* <p>User Token Account: {vault.userTokenAccount}</p>
                        <p>User aToken Account: {vault.userATokenAccount}</p> */}
                        <p>Owner: {vault.owner}</p>
                        <p>User Token Balance: {balances[vault.vaultAccount]?.userTokenBalance || '0'}</p>
                        <p>User aToken Balance: {balances[vault.vaultAccount]?.userATokenBalance || '0'}</p>
                        <p>Vault Token Balance: {balances[vault.vaultAccount]?.vaultTokenBalance || '0'}</p>
                        <div className="button-group" style={{ display: 'flex', gap: '10px' }}>
                            <button className="btn" onClick={() => handleDeposit(vault)}>
                                Deposit
                            </button>
                            <button className="btn" onClick={() => handleWithdraw(vault.vaultAccount)}>
                                Withdraw
                            </button>
                        </div>
                    </div>
                ))}
            </div>
        </div>
    );
};

export default ListVaultsFromRegistry;
