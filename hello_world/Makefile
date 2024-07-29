all: t dev deploy
	echo "finished"
dev:
	cargo build-bpf
t:
	cargo test-bpf
deploy:
	solana program deploy ./target/deploy/hello_world.so --url localhost

create-vault:
	solana program -k user.json invoke --program-id <PROGRAM_ID> --data "00" --accounts '[{"pubkey":"$(solana address -k mint.json)","is_signer":true,"is_writable":true},{"pubkey":"$(solana address -k user.json)","is_signer":true,"is_writable":true},{"pubkey":"SysvarRent111111111111111111111111111111111"}]'
deposit:
	solana program -k user.json invoke --program-id <PROGRAM_ID> --data "01e803000000000000" --accounts '[{"pubkey":"$(solana address -k user.json)","is_signer":true,"is_writable":true},{"pubkey":"$(solana address -k vault.json)","is_signer":false,"is_writable":true}]'
withdraw:
	solana program -k user.json invoke --program-id <PROGRAM_ID> --data "02e803000000000000" --accounts '[{"pubkey":"$(solana address -k vault.json)","is_signer":false,"is_writable":true},{"pubkey":"$(solana address -k user.json)","is_signer":true,"is_writable":trueTo implement a Solana program with multiple external callable functions that accepts funds, locks them into a vault, and mints a 1-to-1 anticoin of the token deposited, you can follow these steps. This guide will provide the complete implementation in Rust for the Solana program and the necessary CLI commands to interact with the program.
