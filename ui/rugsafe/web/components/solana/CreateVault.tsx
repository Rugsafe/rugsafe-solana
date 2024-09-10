import React, { useState, useEffect } from 'react';
import { PublicKey, Keypair, Connection } from '@solana/web3.js';
import { useWallet } from '@solana/wallet-adapter-react';
import { createVault, callFaucet, getTokenBalance } from './transaction-utils';

const LOCALHOST_URL = 'http://127.0.0.1:8899';

const SPL_TOKEN_PROGRAM_ID = 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA';
// const CONTRACT_PROGRAM_ID = 'AVFEXtCiwxuBHuMUsnFGoFB44ymVAbMn3QsN6f6pw5yA';
// const CONTRACT_PROGRAM_ID = 'FobNvbQsK5BAniZC2oJhXakjcPiArpsthTGDnX9eHDVY';
const CONTRACT_PROGRAM_ID = '7kaX5wHo7iyG99uG91aAxnFgi55ZBhSZuGaxp8x3qqDv'
const CreateVault = () => {
    // const [mintPubkey, setMintPubkey] = useState<Keypair | null>(null);
    const [mintPubkey, setMintPubkey] = useState<PublicKey | null>(null);
    const [balance, setBalance] = useState("-");

    const wallet = useWallet();
    const connection = new Connection(LOCALHOST_URL, 'confirmed');

    const programId = new PublicKey(CONTRACT_PROGRAM_ID);

    useEffect(() => {
        if (!mintPubkey) {
            // Generate a new mint public key if it hasn't been set
            // const newMintKeypair = Keypair.generate();
            // const newMintKeypair = new PublicKey("3JR13Th4Lp7Y6nBhj2LP1mMciQG4ZJoT3t9rF2D5xjNq");
            const newMintKeypair = new PublicKey("DG3jdET19heUQjp8fdL54FBvFd5oFWZZjCG8XgmFAHQJ");
            setMintPubkey(newMintKeypair);
            console.log('Generated new mint public key:', newMintKeypair.toBase58());
        }
    }, [mintPubkey]);

    const handleCreateVault = async () => {
        try {
            const txSignature = await createVault(programId, wallet, connection);
            console.log('Transaction successful with signature:', txSignature);
        } catch (error) {
            console.error('Transaction failed', error);
        }
    };

    const handleFaucet = async () => {
        try {
            if (!mintPubkey) {
                console.error('Mint public key is not set');
                return;
            }

            // const mintPublicKey = new PublicKey(mintPubkey.publicKey);
            console.log("INSIDE HANDLE FAUCET, PRIOR: ", mintPubkey)

            // const txSignature = await callFaucet(programId, wallet, connection, mintPubkey);
            const txSignature = await callFaucet(programId, wallet, connection);
            // const txSignature = await callFaucet(programId, wallet, connection, mintPublicKey);
            console.log('Faucet transaction successful with signature:', txSignature);
            fetchBalance(); // Update balance after faucet call
        } catch (error) {
            console.error('Faucet transaction failed', error);
        }
    };

    const fetchBalance = async () => {
        try {
            if (!mintPubkey) {
                console.error('Mint public key is not set');
                return;
            }

            // TODO
            console.log(" mintPubkey.publicKey.toBase58(): ",  mintPubkey.toBase58())
            // const userBalance = await getTokenBalance(connection, wallet, mintPubkey.toBase58());
            const userBalance = await getTokenBalance(connection, wallet, mintPubkey);
            setBalance(userBalance);
            // console.log('Fetched user balance:', userBalance);

        } catch (error) {
            console.error('Failed to fetch balance', error);
        }
    };

    useEffect(() => {
        if (wallet.connected && mintPubkey) {
            fetchBalance();
        }
    }, [wallet.connected, mintPubkey]);

    return (
        <div className="flex flex-col justify-start" style={{ width: '100%', maxWidth: '400px' }}>
            <div className="bg-gray-800 p-6 rounded-lg shadow-lg w-full">
                <h2 className="text-xl font-bold mb-4 text-gray-200">Manage Vaults</h2>

                <div className="flex justify-between">
                    <button
                        className="bg-indigo-600 hover:bg-indigo-700 text-white font-semibold py-2 px-4 rounded-md w-full mr-2"
                        onClick={handleCreateVault}
                        disabled={!wallet.connected}
                    >
                        Create Vault
                    </button>
                    <button
                        className="bg-green-600 hover:bg-green-700 text-white font-semibold py-2 px-4 rounded-md w-full ml-2"
                        onClick={handleFaucet}
                        disabled={!wallet.connected}
                    >
                        Faucet
                    </button>
                </div>
            </div>
        </div>
    );
};

export default CreateVault;
