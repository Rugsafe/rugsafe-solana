# RugSafe Protocol - Solana Programs

![RugSafe Logo](https://rugsafe.io/_next/static/media/logo5.7217ba98.png)

RugSafe is a cutting-edge multichain protocol designed to address and mitigate rug-pull risks in decentralized finance (DeFi). By transforming rugged tokens into opportunities, the protocol leverages cryptographic security measures, economic incentives, and innovative mechanisms to protect users while creating new financial instruments. RugSafe redefines recovery and resilience in DeFi, enabling users to secure their assets, recover losses, and engage in advanced financial operations.

| Status Type          | Status                                                                 |
|----------------------|-------------------------------------------------------------------------|
| **Development Build**| [![Development Build](https://github.com/rugsafe/solana-program/actions/workflows/development.yml/badge.svg)](https://github.com/rugsafe/solana-program/actions/workflows/development.yml) |
| **Issues**           | [![Issues](https://img.shields.io/github/issues/rugsafe/solana-program.svg)](https://github.com/rugsafe/solana-program/issues) |
| **Last Commit**      | [![Last commit](https://img.shields.io/github/last-commit/rugsafe/solana-program.svg)](https://github.com/rugsafe/solana-program/commits/main) |
| **License**          | [![License](https://img.shields.io/badge/license-APACHE-blue.svg)](https://github.com/rugsafe/solana-program/blob/main/LICENSE) |

## Protocol Overview

RugSafe integrates recovery mechanisms and financial instruments to empower DeFi users:

1. **Vault Mechanism**: Securely deposit rugged tokens to receive anti-coins, which inversely correlate to the token’s value.
2. **Perpetual Contracts**: Advanced trading instruments with collateralized leverage and robust liquidation processes.
3. **Decentralized Exchange (DEX)**: Facilitates trading of rugged tokens, anti-coins, and other ecosystem assets.
4. **Rug Detection**: Real-time mechanisms to identify and mitigate potential rug pulls.
5. **Anti-Coin Dynamics**: Implements an inverse logarithmic pegging model to stabilize and hedge rugged token value declines.

## Key Features

### Vaults
- **Token Recovery**: Deposit rugged tokens to mint anti-coins.
- **Anti-Coins**: Inversely pegged to rugged tokens, offering protection and stability.
- **Secure Registry**: Vaults are tracked across blockchains in a unified registry.

### Perpetuals
- **Leverage Trading**: Open long or short positions with collateralized assets.
- **Liquidation Mechanisms**: Secure systems to manage under-collateralized positions.
- **Advanced Collateral Management**: Add or adjust collateral dynamically.

### Decentralized Exchange (DEX)
- **Trading Infrastructure**: Swap rugged tokens, anti-coins, and stable assets.
- **Market Stability**: Enforces inverse logarithmic pegging between rugged tokens and anti-coins.
- **Liquidity Incentives**: Rewards liquidity providers for maintaining balanced markets.

### Rug Detection Mechanisms
- **Liquidity Monitoring**: Detects sudden liquidity changes signaling potential rug pulls.
- **Proactive Interventions**: Executes protective actions such as front-running or sandwiching to secure assets.
- **Mitigation Systems**: Allows users to define automated intents for managing risky positions.

## This Repository

This repository focuses on the Solana implementation of the RugSafe protocol, specifically the **Vaults** and **Perpetuals** programs:

1. **Vaults Program**: Secure rugged token deposits and issue anti-coins.
2. **Perpetuals Program**: Enable leveraged trading and advanced position management.

For details on these modules, see below.

### Vaults Program
- **Deposit**: Secure rugged tokens in vaults.
- **Withdraw**: Retrieve deposited assets.
- **Anti-Coin Issuance**: Mint anti-coins to hedge rugged token losses.

### Perpetuals Program
- **Open Positions**: Initiate leveraged trades (long or short).
- **Manage Collateral**: Dynamically adjust collateral.
- **Liquidation Rules**: Enforce safeguards for under-collateralized positions.



## Quick Start

### Setting Up the Environment

To get started, ensure you have the following tools installed:
- **Rust**: [Install Rust](https://www.rust-lang.org/tools/install)
- **Solana CLI**: [Install Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)

### Build and Test

This repository uses a **workspace** to manage the `rugsafe-perps` and `rugsafe-vaults` programs. The `Makefile` provides convenient targets to streamline development.

#### Folder Structure
```bash
.
├── Makefile
├── Cargo.toml
├── rugsafe-perps
├── rugsafe-vaults
├── tests
└── README.md
```

#### Development Commands

Build the programs:
```bash
$ make dev
```

Run integration tests:
```bash
$ make t
```

## Contributing

We welcome contributions to RugSafe! Join our community and help shape the future of DeFi:
- **Discord**: [Join our community](https://discord.gg/ecMQ2D6nsu)
- **Telegram**: [Join the chat](https://t.me/rugsafe)

## License

RugSafe is released under the [GPL License](LICENSE).

---

**Note**: This repository is under active development and may undergo significant changes. For a detailed understanding of RugSafe, refer to our [white paper](https://rugsafe.io/assets/paper/rugsafe.pdf).
