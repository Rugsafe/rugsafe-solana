https://solana.com/developers/guides/getstarted/setup-local-development

avm use latest  
solana --version
anchor --version
solana-test-validator
solana config set --url localhost
solana config get
solana-keygen new

seed
modify window finger chat session proud myth hub ability obvious explain twelve

```
0xjovis-MBP:solana-dev 0xjovi$ solana-keygen new
Generating a new keypair

For added security, enter a BIP39 passphrase

NOTE! This passphrase improves security of the recovery seed phrase NOT the
keypair file itself, which is stored as insecure plain text

BIP39 Passphrase (empty for none): 

Wrote new keypair to /Users/0xjovi/.config/solana/id.json
===============================================================================
pubkey: DZ92eKdkjqkToFwcGCHDRwbJwiWYKe57jZgMV3gSuhFT
===============================================================================
Save this seed phrase and your BIP39 passphrase to recover your new keypair:
modify window finger chat session proud myth hub ability obvious explain twelve
===============================================================================
```

solana config set -k ~/.config/solana/id.json
solana airdrop 2
solana balance


https://solana.com/developers/guides/getstarted/local-rust-hello-world
cargo init hello_world --lib
cargo add solana-program

cargo build-bpf

to resolve this Error Simply Run 2 Commands
1- cargo add solana-program@=1.17.0
Then Run
2- cargo update -p solana-program

After Simply Run : anchor build . and you Good To Go

cargo add solana-program@=1.17.25
cargo update -p solana-program
anchor build

cargo build-sbf --tools-version v1.39
cargo +stable build-sbf

 sh -c "$(curl -sSfL https://release.solana.com/v1.18.18/install)"
solana config set -k ~/.config/solana/id.json
solana config set -k /Users/0xjovi/.config/solana/id.json

solana program deploy ./target/deploy/hello_world.so


solana-keygen new -o /Users/0xjovi/.config/solana/id.json

solana airdrop 2 -k  /Users/0xjovi/.config/solana/id.json --url localhost

solana balance --url localhost

solana program deploy ./target/deploy/hello_world.so --url localhost

CoNXM11qc8LmGhTLKb7i75HhSP4rgc3eNcyNHgYn377o

solana-ledger-tool program run -l test-ledger -e debugger target/deploy/helloworld.so

solana config set --url localhost

solana transfer recipient_address 1.23 --from ~/.config/solana/id.json
solana transfer 9hpcmyrpfSzn2T4XmioWzQ8xvAfFhSjxuTv8ioDKKT1P 0.11 --from ~/.config/solana/id.json


# goal

//create vault
// deposit tokena -> receive rtokena
// withdraw token a -> deposits rtokena + burns rtokena?
// burn rtokena -> receives btokena

// RToken
// underlying tokena
// has a Safe?
// whenever Rtoken is withdrawn against
//(attempting to claim the underlying token again,
//it distributes a percentage of the underlying token (or rtoken) to vaults? liquidity providers?)

// Vault
// takes tokena as a deposit
// mints rtokena in exchange for tokena
// burns rtokena and mints btokena

// Pool
// tokena is always sol
