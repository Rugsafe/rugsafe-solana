import React, { useState, useEffect } from 'react';
import { PublicKey, Connection } from '@solana/web3.js';
import { useWallet } from '@solana/wallet-adapter-react';
import { deposit } from '../solana/transaction-utils'; // Adjust the import path accordingly
import { getAssociatedTokenAddress, TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID} from '@solana/spl-token';

const LOCALHOST_URL = 'http://127.0.0.1:8899';
const CONTRACT_PROGRAM_ID = 'AVFEXtCiwxuBHuMUsnFGoFB44ymVAbMn3QsN6f6pw5yA';

const ListVaults = () => {
    const [vaults, setVaults] = useState<Array<{pubkey: string, mint: string, userTokenAccount: string, userATokenAccount: string}>>([]);
    const [activeTab, setActiveTab] = useState('deposit');
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
                    const mint = getMintFromData(data);
                    const userTokenAccount = getUserTokenAccountFromData(data);
                    const userATokenAccount = getUserATokenAccountFromData(data);

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
        return true; // Placeholder - replace with your own logic
    };

    const getMintFromData = (data: Buffer): string => {
        return 'YOUR_MINT_PUBKEY'; // Replace with actual logic to extract this
    };

    const getUserTokenAccountFromData = (data: Buffer): string => {
        return 'USER_TOKEN_ACCOUNT'; // Replace with actual logic to extract this
    };

    const getUserATokenAccountFromData = (data: Buffer): string => {
        return 'USER_ATOKEN_ACCOUNT'; // Replace with actual logic to extract this
    };

    const handleDeposit = async (vault: {
        pubkey: string,
        mint: string,
        userTokenAccount: string,
        userATokenAccount: string
    }) => {
        try {
            const programId = new PublicKey(CONTRACT_PROGRAM_ID);
            const mintTokenAPubkey = new PublicKey(vault.mint);
            const mintATokenAPubkey = new PublicKey(vault.userATokenAccount);
            
            const vaultPubkey = new PublicKey(vault.pubkey);
            
            const userTokenAPubkey = new PublicKey(vault.userTokenAccount);
            
            const userATokenAPubkey = new PublicKey(vault.userATokenAccount);
    
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

    const handleWithdraw = async (vault: {
        pubkey: string, 
        userTokenAccount: string
    }) => {
        try {
            console.log(`Withdraw clicked for vault: ${vault.pubkey}`);
            // Implement withdraw logic here
            // Make sure to handle the withdrawal on-chain via Solana
        } catch (error) {
            console.error('Withdraw failed:', error);
        }
    };

    return (
        <div className="min-h-screen bg-gradient-to-br from-pink-500 to-purple-600 p-6">
            <div className="w-full max-w-4xl mx-auto bg-gray-900/80 text-white backdrop-blur-sm rounded-lg overflow-hidden">
                <div className="p-6">
                    <div className="flex items-center justify-between mb-6">
                        <button className="text-white flex items-center">
                            <svg xmlns="http://www.w3.org/2000/svg" className="h-6 w-6 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" />
                            </svg>
                            Back
                        </button>
                        <div className="bg-yellow-500 rounded-full p-2">
                            <svg xmlns="http://www.w3.org/2000/svg" className="h-6 w-6 text-white" viewBox="0 0 20 20" fill="currentColor">
                                <path d="M8.433 7.418c.155-.103.346-.196.567-.267v1.698a2.305 2.305 0 01-.567-.267C8.07 8.34 8 8.114 8 8c0-.114.07-.34.433-.582zM11 12.849v-1.698c.22.071.412.164.567.267.364.243.433.468.433.582 0 .114-.07.34-.433.582a2.305 2.305 0 01-.567.267z" />
                                <path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm1-13a1 1 0 10-2 0v.092a4.535 4.535 0 00-1.676.662C6.602 6.234 6 7.009 6 8c0 .99.602 1.765 1.324 2.246.48.32 1.054.545 1.676.662v1.941c-.391-.127-.68-.317-.843-.504a1 1 0 10-1.51 1.31c.562.649 1.413 1.076 2.353 1.253V15a1 1 0 102 0v-.092a4.535 4.535 0 001.676-.662C13.398 13.766 14 12.991 14 12c0-.99-.602-1.765-1.324-2.246A4.535 4.535 0 0011 9.092V7.151c.391.127.68.317.843.504a1 1 0 101.511-1.31c-.563-.649-1.413-1.076-2.354-1.253V5z" clipRule="evenodd" />
                            </svg>
                        </div>
                    </div>
                    <h2 className="text-4xl font-bold text-center mb-2">DAI</h2>
                    <p className="text-sm text-gray-400 text-center mb-4">0x028eC7330ff87667b6dfb0D94b954c820195336c</p>
                    <div className="flex justify-center space-x-2 mb-6">
                        <button className="bg-gray-700 text-white px-3 py-1 rounded text-xs">Dai Stablecoin</button>
                        <button className="bg-gray-700 text-white px-3 py-1 rounded text-xs">Ethereum</button>
                    </div>
                    <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6">
                        <div>
                            <p className="text-sm text-gray-400">Total deposited, yvDAI-1</p>
                            <p className="text-2xl font-bold">3,910,665.01</p>
                            <p className="text-sm text-gray-400">$3,908,944.32</p>
                        </div>
                        <div>
                            <p className="text-sm text-gray-400">Historical APY</p>
                            <p className="text-2xl font-bold">6.02%</p>
                            <p className="text-sm text-gray-400">Est. APY: 5.57% → 6.94%</p>
                        </div>
                        <div>
                            <p className="text-sm text-gray-400">Value in DAI</p>
                            <p className="text-2xl font-bold">0.00</p>
                            <p className="text-sm text-gray-400">$0.00</p>
                        </div>
                        <div>
                            <p className="text-sm text-gray-400">Extra earned, dYFI</p>
                            <p className="text-2xl font-bold">0.00</p>
                            <p className="text-sm text-gray-400">$0.00</p>
                        </div>
                    </div>
                    <div className="mb-6">
                        <div className="flex border-b border-gray-700">
                            <button 
                                className={`flex-1 py-2 text-center ${activeTab === 'deposit' ? 'border-b-2 border-purple-500' : ''}`}
                                onClick={() => setActiveTab('deposit')}
                            >
                                Deposit
                            </button>
                            <button 
                                className={`flex-1 py-2 text-center ${activeTab === 'withdraw' ? 'border-b-2 border-purple-500' : ''}`}
                                onClick={() => setActiveTab('withdraw')}
                            >
                                Withdraw
                            </button>
                            <button 
                                className={`flex-1 py-2 text-center ${activeTab === 'boost' ? 'border-b-2 border-purple-500' : ''}`}
                                onClick={() => setActiveTab('boost')}
                            >
                                veYFI BOOST
                            </button>
                        </div>
                    </div>
                    {activeTab === 'deposit' && (
                        <div>
                            <div className="grid grid-cols-2 gap-4 mb-4">
                                <div>
                                    <p className="text-sm mb-2">From wallet</p>
                                    <div className="flex items-center space-x-2 bg-gray-800 p-2 rounded">
                                        <div className="bg-yellow-500 rounded-full p-1">
                                            <svg xmlns="http://www.w3.org/2000/svg" className="h-4 w-4 text-white" viewBox="0 0 20 20" fill="currentColor">
                                                <path d="M8.433 7.418c.155-.103.346-.196.567-.267v1.698a2.305 2.305 0 01-.567-.267C8.07 8.34 8 8.114 8 8c0-.114.07-.34.433-.582zM11 12.849v-1.698c.22.071.412.164.567.267.364.243.433.468.433.582 0 .114-.07.34-.433.582a2.305 2.305 0 01-.567.267z" />
                                                <path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm1-13a1 1 0 10-2 0v.092a4.535 4.535 0 00-1.676.662C6.602 6.234 6 7.009 6 8c0 .99.602 1.765 1.324 2.246.48.32 1.054.545 1.676.662v1.941c-.391-.127-.68-.317-.843-.504a1 1 0 10-1.51 1.31c.562.649 1.413 1.076 2.353 1.253V15a1 1 0 102 0v-.092a4.535 4.535 0 001.676-.662C13.398 13.766 14 12.991 14 12c0-.99-.602-1.765-1.324-2.246A4.535 4.535 0 0011 9.092V7.151c.391.127.68.317.843.504a1 1 0 101.511-1.31c-.563-.649-1.413-1.076-2.354-1.253V5z" clipRule="evenodd" />
                                            </svg>
                                        </div>
                                        <span>FLOCKA</span>
                                    </div>
                                </div>
                                <div>
                                    <p className="text-sm mb-2">Amount</p>
                                    <input type="number" placeholder="0" className="w-full bg-gray-800 border border-gray-700 rounded p-2" />
                                </div>
                                <div>
                                    <p className="text-sm mb-2">To vault</p>
                                    <div className="flex items-center space-x-2 bg-gray-800 p-2 rounded">
                                        <div className="bg-blue-500 rounded-full p-1">
                                            <svg xmlns="http://www.w3.org/2000/svg" className="h-4 w-4 text-white" viewBox="0 0 20 20" fill="currentColor">
                                                <path fillRule="evenodd" d="M5 9V7a5 5 0 0110 0v2a2 2 0 012 2v5a2 2 0 01-2 2H5a2 2 0 01-2-2v-5a2 2 0 012-2zm8-2v2H7V7a3 3 0 016 0z" clipRule="evenodd" />
                                            </svg>
                                        </div>
                                        <span>rugFlocka-1</span>
                                    </div>
                                </div>
                                <div>
                                    <p className="text-sm mb-2">You will receive</p>
                                    <input type="text" disabled value="0" className="w-full bg-gray-800 border border-gray-700 rounded p-2" />
                                </div>
                            </div>
                            <button 
    onClick={() => vaults.length > 0 ? handleDeposit(vaults[0]) : null}
    className="w-full bg-purple-600 hover:bg-purple-700 text-white font-bold py-2 px-4 rounded"
    disabled={vaults.length === 0}
>
    Deposit
</button>
                            <div className="mt-4 p-4 bg-green-900/50 rounded-lg">
                                <p className="text-sm">
                                    This Vault has an active veYFI gauge which boosts your APY from 0.15% to 1.52% depending on the veYFI
                                    you have locked. Simply deposit and stake to start earning.
                                </p>
                                <p className="text-sm mt-2">Learn more about veYFI rewards in the FAQ.</p>
                            </div>
                        </div>
                    )}
                    {activeTab === 'withdraw' && (
                        <div>
                            <div className="grid grid-cols-2 gap-4 mb-4">
                                <div>
                                    <p className="text-sm mb-2">Vault Address</p>
                                    <div className="flex items-center space-x-2 bg-gray-800 p-2 rounded">
                                        <span>{vaults[0]?.pubkey}</span> {/* Use correct vault */}
                                    </div>
                                </div>
                                <div>
                                    <p className="text-sm mb-2">Withdraw Amount</p>
                                    <input type="number" placeholder="0" className="w-full bg-gray-800 border border-gray-700 rounded p-2" />
                                </div>
                            </div>
                            <button 
                                onClick={() => handleWithdraw(vaults[0])} // Assuming vaults[0] for simplicity, update to correct vault
                                className="w-full bg-purple-600 hover:bg-purple-700 text-white font-bold py-2 px-4 rounded"
                            >
                                Withdraw
                            </button>
                        </div>
                    )}
                </div>
            </div>
        </div>
    );
};

export default ListVaults;

