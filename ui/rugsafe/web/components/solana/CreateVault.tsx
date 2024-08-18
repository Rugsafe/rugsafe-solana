import React, { useState, useEffect } from 'react';
import { PublicKey, Keypair, Connection } from '@solana/web3.js';
import { useWallet } from '@solana/wallet-adapter-react';
import { createVault, callFaucet, getTokenBalance } from './transaction-utils';

const LOCALHOST_URL = 'http://127.0.0.1:8899';

const SPL_TOKEN_PROGRAM_ID = 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA';
// const CONTRACT_PROGRAM_ID = 'AVFEXtCiwxuBHuMUsnFGoFB44ymVAbMn3QsN6f6pw5yA';
// const CONTRACT_PROGRAM_ID = 'FobNvbQsK5BAniZC2oJhXakjcPiArpsthTGDnX9eHDVY';
const CONTRACT_PROGRAM_ID = 'FobNvbQsK5BAniZC2oJhXakjcPiArpsthTGDnX9eHDVY'
const CreateVault = () => {
    // const [mintPubkey, setMintPubkey] = useState<Keypair | null>(null);
    const [mintPubkey, setMintPubkey] = useState<PublicKey | null>(null);
    const [balance, setBalance] = useState(0);

    const wallet = useWallet();
    const connection = new Connection(LOCALHOST_URL, 'confirmed');

    const programId = new PublicKey(CONTRACT_PROGRAM_ID);

    useEffect(() => {
        if (!mintPubkey) {
            // Generate a new mint public key if it hasn't been set
            // const newMintKeypair = Keypair.generate();
            const newMintKeypair = new PublicKey("3JR13Th4Lp7Y6nBhj2LP1mMciQG4ZJoT3t9rF2D5xjNq");
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
        <div>
            <button className="btn" onClick={handleCreateVault} disabled={!wallet.connected}>
                Create Vault
            </button>
            <button className="btn" onClick={handleFaucet} disabled={!wallet.connected}>
                Get Tokens from Faucet
            </button>
            <div>
                <h3>Your Token Balance: {balance}</h3>
                {mintPubkey && (
                    <p>Mint Public Key: {mintPubkey.toBase58()}</p>
                )}
            </div>
        </div>
    );
};

export default CreateVault;
