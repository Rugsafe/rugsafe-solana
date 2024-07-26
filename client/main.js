const solanaWeb3 = require('@solana/web3.js');
const fs = require('fs');
const http = require('http');

// Load the keypair
const payerKeypair = solanaWeb3.Keypair.fromSecretKey(
    new Uint8Array(JSON.parse(fs.readFileSync('/Users/0xjovi/.config/solana/id.json')))
);

console.log(payerKeypair)

// Connection to local Solana cluster
const connection = new solanaWeb3.Connection('http://127.0.0.1:8899', 'confirmed');
console.log(connection)

// Program ID
const programId = new solanaWeb3.PublicKey('AVFEXtCiwxuBHuMUsnFGoFB44ymVAbMn3QsN6f6pw5yA');
console.log(programId)

// Direct HTTP request to check connectivity
async function checkConnection() {
    return new Promise((resolve, reject) => {
        http.get('http://127.0.0.1:8899', (res) => {
            let data = '';
            res.on('data', (chunk) => {
                data += chunk;
            });
            res.on('end', () => {
                console.log('HTTP check response:', data);
                resolve(data);
            });
        }).on('error', (err) => {
            console.error('HTTP check failed:', err);
            reject(err);
        });
    });
}

// Create a transaction
async function main() {
    try {
        console.log("Checking connection to the validator...");
        const response = await checkConnection();
        console.log("Validator response:", response);

        console.log("Attempting to fetch recent blockhash...");
        let recentBlockhash = await connection.getRecentBlockhash();
        console.log("Recent blockhash fetched successfully:", recentBlockhash);

        let transaction = new solanaWeb3.Transaction().add(
            new solanaWeb3.TransactionInstruction({
                keys: [],
                programId,
                data: Buffer.alloc(0), // Add any necessary instruction data here
            })
        );
        transaction.recentBlockhash = recentBlockhash.blockhash;
        transaction.feePayer = payerKeypair.publicKey;

        console.log("Signing transaction...");
        transaction.sign(payerKeypair);

        console.log("Sending transaction...");
        let signature = await solanaWeb3.sendAndConfirmTransaction(connection, transaction, [payerKeypair]);
        console.log('Transaction signature', signature);
    } catch (error) {
        console.error('Error sending transaction:', error);
        if (error.message.includes('fetch failed')) {
            console.error('Ensure the Solana test validator is running and accessible.');
        }
    }
}

main().catch(err => {
    console.error('Error in main:', err);
});
