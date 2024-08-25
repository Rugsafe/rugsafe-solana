import React, { useState, useEffect } from 'react';
import { PublicKey, Connection, Keypair } from '@solana/web3.js';
import { useWallet } from '@solana/wallet-adapter-react';
import { fetchVaultRegistry, deposit } from './solana/transaction-utils';
import { getAssociatedTokenAddress} from '@solana/spl-token';
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
    
            console.log("vaultRegistry.vaults", vaultRegistry.vaults);
    
            const formattedVaults = vaultRegistry.vaults.map((vault) => ({
                vaultAccount: vault.vaultAccount.toBase58(),
                mintTokenAAccount: vault.mintTokenA.toBase58(), // Using the correct field name for Token A mint
                mintATokenAAccount: vault.mintATokenA.toBase58(), // Using the correct field name for ATokenA mint
                owner: vault.owner.toBase58(),
            }));
    
            console.log("formattedVaults", formattedVaults);
    
            setVaults(formattedVaults);
            await fetchBalances(formattedVaults);
        } catch (error) {
            console.error('Error fetching vault registry:', error);
        }
    };
    // const fetchBalances = async (vaults: any) => {
    //     const newBalances = {};
    
    //     for (const vault of vaults) {
    //         try {
    //             console.log(`Fetching balances for vault ${vault.vaultAccount}`);
    //             console.log(`Vault Account: ${vault.vaultAccount}`);
    
    //             if (!vault.userTokenAccount || !vault.userATokenAccount) {
    //                 console.error(`Vault ${vault.vaultAccount} is missing token accounts`);
    //                 continue;
    //             }
    
    //             const userTokenAccountBalance = await connection.getTokenAccountBalance(new PublicKey(vault.userTokenAccount));
    //             const userATokenAccountBalance = await connection.getTokenAccountBalance(new PublicKey(vault.userATokenAccount));
    //             const vaultTokenAccountBalance = await connection.getTokenAccountBalance(new PublicKey(vault.vaultAccount));
    
    //             console.log(`User Token Balance: ${userTokenAccountBalance.value.uiAmount}`);
    //             console.log(`User aToken Balance: ${userATokenAccountBalance.value.uiAmount}`);
    //             console.log(`Vault Token Balance: ${vaultTokenAccountBalance.value.uiAmount}`);
    
    //             newBalances[vault.vaultAccount] = {
    //                 userTokenBalance: userTokenAccountBalance.value.amount,
    //                 userATokenBalance: userATokenAccountBalance.value.amount,
    //                 vaultTokenBalance: vaultTokenAccountBalance.value.amount,
    //             };
    //         } catch (error) {
    //             console.error(`Error fetching balances for vault ${vault.vaultAccount}:`, error);
    //         }
    //     }
    
    //     setBalances(newBalances);
    // };

    const fetchBalances = async (vaults: any) => {
        const newBalances: { [key: string]: { userTokenBalance: string,  vaultTokenBalance: string} } = {};

    
        for (const vault of vaults) {
            try {
                console.log(`vault ${vault}`);
                console.log(`Fetching balances for vault ${vault.vaultAccount}`);
                console.log(`Vault Account: ${vault.vaultAccount}`);
    
                // Ensure token accounts exist before attempting to fetch balances
                // if (!vault.userTokenAccount || !vault.userATokenAccount) {
                //     console.error(`Vault ${vault.vaultAccount} is missing token accounts`);
                //     continue; // Skip this vault and move on to the next
                // }
    
                const userTokenAccountBalance = await connection.getTokenAccountBalance(new PublicKey("43MJ8hVyFQBoNw2Qm8WnYVfrZkfEVjGUNnRDuBjTj9kg"));
                // const userATokenAccountBalance = await connection.getTokenAccountBalance(new PublicKey(vault.userATokenAccount));
                const vaultTokenAccountBalance = await connection.getTokenAccountBalance(new PublicKey("Dof5p3fEhZhXttrPeEPiKwLoac5ftRyJJnma24ZYF4qZ"));

                console.log(`userTokenAccountBalance: ${userTokenAccountBalance.value.uiAmount}`);
                console.log(`vaultTokenAccountBalance: ${userTokenAccountBalance.value.uiAmount}`);
                // 
                // const userTokenAccountBalance2 = await connection.getTokenAccountBalance(new PublicKey(
                //     // "3JR13Th4Lp7Y6nBhj2LP1mMciQG4ZJoT3t9rF2D5xjNq" // static account
                //     // "3JR13Th4Lp7Y6nBhj2LP1mMciQG4ZJoT3t9rF2D5xjNq" // static account
                //     "DG3jdET19heUQjp8fdL54FBvFd5oFWZZjCG8XgmFAHQJ"
                // ));

                
    
                // console.log(`User Token Balance: ${userTokenAccountBalance.value.uiAmount}`);
                // console.log(`User aToken Balance: ${userATokenAccountBalance.value.uiAmount}`);
                // console.log(`Vault Token Balance: ${vaultTokenAccountBalance.value.uiAmount}`);
                // console.log(`Vault Token Balance: ${userTokenAccountBalance2.value.uiAmount}`);

                newBalances[vault.vaultAccount] = {
                    userTokenBalance: userTokenAccountBalance.value.amount,
                    // userATokenBalance: userATokenAccountBalance.value.amount,
                    vaultTokenBalance: vaultTokenAccountBalance.value.amount,
                };
            } catch (error) {
                console.error(`Error fetching balances for vault ${vault.vaultAccount}:`, error);
            }
        }
    
        // window.alert("setting balances")
        console.log("newBalances:", newBalances)
        setBalances(newBalances);
    };
    

    // const handleDepositx = async (vault: any) => {
    //     try {
    //         const programId = new PublicKey(CONTRACT_PROGRAM_ID);

    //         // NOTE: needs to receive token a (the fauceted accounts) - so needs to be a token account for the token a mint
    //         const vaultPubkey = new PublicKey(vault.vaultAccount);

    //         const mintPubkey = new PublicKey(vault.mintAccount);
    //         // const mintPubkey = new PublicKey("DG3jdET19heUQjp8fdL54FBvFd5oFWZZjCG8XgmFAHQJ")
    //         const depositAmount = 1;

    //         // NOTE:  Generate key pairs for the user's token and aToken accounts
    //         // const userTokenAPubkey = Keypair.generate().publicKey;
    //         // const userTokenAPubkey = new PublicKey("3JR13Th4Lp7Y6nBhj2LP1mMciQG4ZJoT3t9rF2D5xjNq");
    //         const userTokenAPubkey = new PublicKey("DG3jdET19heUQjp8fdL54FBvFd5oFWZZjCG8XgmFAHQJ");

    //         // NOTE: THIS IS THE ACCOUNT THAT RECEIVES THE ANTICOINS -- WHICH SHOULD BE AN ACCOUNT FOR THE USER
    //         // const userATokenAPubkey = Keypair.generate();
    //         const userATokenAPubkey = await getAssociatedTokenAddress(
    //             mintPubkey,
    //             wallet.publicKey as PublicKey
    //         );

    //         console.log("userATokenAPubkey:,", userATokenAPubkey)
            
            
    //         // debugger;
            
            
    //         // const signature = await deposit(
    //         //     programId,
    //         //     mintPubkey,
    //         //     vaultPubkey,
    //         //     userTokenAPubkey,
    //         //     userATokenAPubkey,
    //         //     depositAmount,
    //         //     wallet,
    //         //     connection
    //         // );

    //         // console.log('Deposit transaction signature:', signature);
    //         // await fetchBalances(vaults); // Refresh balances after deposit
    //     } catch (error) {
    //         console.error('Deposit failed:', error);
    //     }
    // };

    // const handleDepositxx = async (vault: any) => {
    //     try {
    //         const programId = new PublicKey(CONTRACT_PROGRAM_ID);
    
    //         // Retrieve vault's public key and associated mints
    //         const vaultPubkey = new PublicKey(vault.vaultAccount);
    //         const mintTokenAPubkey = new PublicKey(vault.mintAccount); // TokenA Mint

    //         // Generate a new key pair for the aTokenA mint
    //         // const mintATokenAPubkey = new PublicKey("FSrN9rrZNvrtr8163PhZxSzYq7utytPKsVTH3Ph5LoD7"); // Replace with the actual aTokenA mint address
    //         const mintATokenAKeypair = Keypair.generate();
    //         const mintATokenAPubkey = mintATokenAKeypair.publicKey; // aTokenA Mint

    //         const depositAmount = 1;
    
    //         // User's TokenA account (the account holding the token the user is depositing)
    //         // const userTokenAPubkey = new PublicKey("DG3jdET19heUQjp8fdL54FBvFd5oFWZZjCG8XgmFAHQJ"); // Replace with the actual user's TokenA account
    //         const userTokenAPubkey = new PublicKey("5Wb8FXJ5PPe8GDb1us3XEXGzTFTW4MnDrMBKHK36RhAE")
    //         // const userTokenAPubkey = new PublicKey("6hBKBJ6vw8zXDAUbz8HFbdZNhEhpXco56SGQNwpvidAq")
            
    //         // User's aTokenA account (the account that will receive aTokens)
    //         const userATokenAPubkey = await getAssociatedTokenAddress(
    //             mintATokenAPubkey,
    //             wallet.publicKey as PublicKey
    //         );
    
    //         console.log("userATokenAPubkey:", userATokenAPubkey.toString());
    
    //         const signature = await deposit(
    //             programId,
    //             mintTokenAPubkey,

    //             // mintATokenAPubkey,
    //             mintATokenAKeypair,

    //             vaultPubkey,
    //             userTokenAPubkey,
    //             userATokenAPubkey,
    //             depositAmount,
    //             wallet,
    //             connection
    //         );
    
    //         console.log('Deposit transaction signature:', signature);
    //         await fetchBalances(vaults); // Refresh balances after deposit
    //     } catch (error) {
    //         console.error('Deposit failed:', error);
    //     }
    // };

    const handleDeposit = async (vault: any) => {
        try {
            const programId = new PublicKey(CONTRACT_PROGRAM_ID);
    
            // Retrieve vault's public key and associated mints
            console.log("vault: ", vault)
            const vaultPubkey = new PublicKey(vault.vaultAccount);
            const mintTokenAPubkey = new PublicKey(vault.mintTokenAAccount); // TokenA Mint
            const mintATokenAPubkey = new PublicKey(vault.mintATokenAAccount); // TokenA Mint

            console.log("vault", vault)
            // const mintTokenAPubkey = new PublicKey("DG3jdET19heUQjp8fdL54FBvFd5oFWZZjCG8XgmFAHQJ")
            // const mintATokenAPubkey = new PublicKey("6hBKBJ6vw8zXDAUbz8HFbdZNhEhpXco56SGQNwpvidAq"); // ATokenA Mint
            console.log("mintATokenAPubkey", mintATokenAPubkey)
            // const mintATokenAKeypair = Keypair.generate();
            // const mintATokenAPubkey = mintATokenA'Keypair.publicKey; // aTokenA Mint
            // const mintATokenAPubkey = new PublicKey("6hBKBJ6vw8zXDAUbz8HFbdZNhEhpXco56SGQNwpvidAq");
            const depositAmount = 1;
    
            // Get the user's TokenA account
            // const userTokenAPubkey = await getAssociatedTokenAddress(
            //     mintTokenAPubkey,
            //     wallet.publicKey as PublicKey
            // );

            const userTokenAPubkey = new PublicKey("43MJ8hVyFQBoNw2Qm8WnYVfrZkfEVjGUNnRDuBjTj9kg")
            
            // User's aTokenA account (the account that will receive aTokens)
            const userATokenAPubkey = await getAssociatedTokenAddress(
                mintATokenAPubkey,
                wallet.publicKey as PublicKey
            );
    
            console.log("userTokenAPubkey:", userTokenAPubkey.toString());
            console.log("userATokenAPubkey:", userATokenAPubkey.toString());
    
            const signature = await deposit(
                programId,
                mintTokenAPubkey,
                // mintATokenAKeypair,
                mintATokenAPubkey,
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
                    <p>Mint Token A Account: {vault.mintTokenAAccount}</p> {/* Updated field name */}
                    <p>Mint AToken A Account: {vault.mintATokenAAccount}</p> {/* Updated field name */}
                    <p>Owner: {vault.owner}</p>
                    <p>User Token Balance: {balances[vault.vaultAccount]?.userTokenBalance || '-'}</p>
                    <p>User aToken Balance: {balances[vault.vaultAccount]?.userATokenBalance || '-'}</p>
                    <p>Vault Token Balance: {balances[vault.vaultAccount]?.vaultTokenBalance || '-'}</p>
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
