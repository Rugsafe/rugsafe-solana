dev:
	cargo build-bpf
t:
	cargo test-bpf --package rugsafe --test main -- test_integration_perps_vaults --nocapture
