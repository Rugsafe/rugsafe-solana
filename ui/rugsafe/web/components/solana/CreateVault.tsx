import React, { useState } from 'react';
import { PublicKey, Keypair, Connection } from '@solana/web3.js';
import { useWallet, useConnection } from '@solana/wallet-adapter-react';
import { createVault } from './transaction-utils';

const LOCALHOST_URL = 'http://127.0.0.1:8899';

// Generate Keypairs for testing
// const mintKeypair = Keypair.generate();
// const ownerKeypair = Keypair.generate();
// Hardcoded Program IDs
const SPL_TOKEN_PROGRAM_ID = 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA';
const CONTRACT_PROGRAM_ID = 'AVFEXtCiwxuBHuMUsnFGoFB44ymVAbMn3QsN6f6pw5yA';



// console.log("Mint PublicKey:", mintKeypair.publicKey.toBase58());
// console.log("Owner PublicKey:", ownerKeypair.publicKey.toBase58());

const CreateVault = () => {
    // const [mintPubkey, setMintPubkey] = useState(mintKeypair.publicKey.toBase58());
    // const [ownerPubkey, setOwnerPubkey] = useState(ownerKeypair.publicKey.toBase58());
    const [mintPubkey, setMintPubkey] = useState('');
    const [ownerPubkey, setOwnerPubkey] = useState('');
    

    const wallet = useWallet();


    // const { connection } = useConnection();
    const connection = new Connection(LOCALHOST_URL, 'confirmed');
  
  
    const programId = new PublicKey('AVFEXtCiwxuBHuMUsnFGoFB44ymVAbMn3QsN6f6pw5yA'); // Replace with your program ID
    // const programId = new PublicKey('AgpAVUCyAdNJPj8XqUzMkcGn42jBnjP7XdpNVNHuAYZW')
    // const programId = new PublicKey('83jZLCcVhfhHygUyGCjLT5EMBczfzBU2DYBEuF3bPVvs')
    // const programId = new PublicKey('Fx7t2guBeTJnhd4qNSRxrNQ8Qab1uGNMVw4VksX1TQ74') // hello world works
    const handleCreateVault = async () => {
        try {
            const txSignature = await createVault(programId, wallet, connection);
            console.log('Transaction successful with signature:', txSignature);
        } catch (error) {
            console.error('Transaction failed', error);
        }
    };

    return (
        <div>
            <input
                type="text"
                value={mintPubkey}
                onChange={(e) => setMintPubkey(e.target.value)}
                placeholder="Mint PublicKey"
            />
            <input
                type="text"
                value={ownerPubkey}
                onChange={(e) => setOwnerPubkey(e.target.value)}
                placeholder="Owner PublicKey"
            />
            <br/>
            <button className="btn" onClick={handleCreateVault} disabled={!wallet.connected}>
                Create Vault
            </button>
        </div>
    );
};

export default CreateVault;
