# Soroban Contract Templates

A curated collection of production-ready Soroban smart contract templates. These templates help developers quickly bootstrap common use cases on Soroban (Stellar's smart contract platform) for DeFi, payments, governance, and more.

## 🚀 Quick Start

```bash
# Clone the repository
git clone https://github.com/Fidelis900/soroban-starter-kit.git
cd soroban-starter-kit

# Build a contract (example: token)
cd contracts/token
soroban contract build

# Deploy to testnet
./scripts/deploy.sh testnet

# Run tests
cargo test
```

## 📦 Contract Templates

| Template | Description | Use Cases | Status |
|----------|-------------|-----------|---------|
| **Token** | Custom fungible token with mint/burn/admin controls | DeFi tokens, governance tokens, utility tokens | ✅ Complete |
| **Escrow** | Two-party escrow with timeout and refund mechanism | P2P trading, service payments, milestone payments | ✅ Complete |

### Token Contract Features
- **Standard Interface**: Full Soroban token compatibility
- **Administrative Controls**: Mint, burn, and admin management
- **Metadata Support**: Name, symbol, and decimals
- **Allowance System**: Approve and transfer_from functionality
- **Event Emission**: All operations emit events for tracking
- **Error Handling**: Custom error types for better debugging

### Escrow Contract Features
- **Two-Party Security**: Secure buyer-seller transactions
- **Deadline Protection**: Automatic refunds after deadline
- **Arbiter Support**: Third-party dispute resolution
- **State Management**: Clear transaction lifecycle
- **Token Agnostic**: Works with any Soroban token
- **Event Emission**: All operations emit events for tracking

Each template includes:
- ✅ Complete contract implementation
- ✅ Comprehensive unit tests (8+ test cases each)
- ✅ Deployment scripts with examples
- ✅ Usage examples and documentation

## 🛠 Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- [Soroban CLI](https://soroban.stellar.org/docs/getting-started/setup#install-the-soroban-cli)
- [Docker](https://www.docker.com/) (for local Stellar node)

## 📖 Usage

### Building Contracts

```bash
cd contracts/[template-name]
soroban contract build
```

### Running Tests

```bash
cd contracts/[template-name]
cargo test
```

### Deploying to Testnet

```bash
cd contracts/[template-name]
./scripts/deploy.sh testnet
```

### Local Development

Start a local Stellar node with Soroban RPC:

```bash
docker compose up stellar-node
```

## 🤝 Contributing

We welcome contributions! Please:
- Add new contract templates following the existing structure
- Include comprehensive tests for all functionality
- Provide clear documentation and usage examples
- Follow Rust and Soroban best practices

## 📚 Resources

- [Soroban Documentation](https://soroban.stellar.org/docs)
- [Stellar Developer Discord](https://discord.gg/stellardev)
- [Soroban Examples](https://github.com/stellar/soroban-examples)
- [Freighter Wallet](https://freighter.app/)
- [Stellar Laboratory](https://laboratory.stellar.org/)

## 📄 License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

---

**Ready to build on Soroban?** Start with any template and customize it for your use case! 🚀