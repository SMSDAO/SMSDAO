# Getting Started with SMSDAO

This guide will help you get up and running with SMSDAO quickly.

## 📋 Prerequisites

Before you begin, ensure you have the following installed:

- **Rust**: Version 1.70 or higher
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- **Solana CLI**: Latest stable version
  ```bash
  sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
  ```

- **Anchor Framework**: Version 0.28 or higher
  ```bash
  cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
  avm install latest
  avm use latest
  ```

- **Node.js**: Version 18 or higher (for testing and deployment scripts)
  ```bash
  # Using nvm (recommended)
  nvm install 18
  nvm use 18
  ```

## 🔧 Installation

### 1. Clone the Repository

```bash
git clone https://github.com/SMSDAO/SMSDAO.git
cd SMSDAO
```

### 2. Install Dependencies

```bash
# Install Rust dependencies
cargo build

# If using JavaScript/TypeScript clients
npm install
# or
yarn install
```

### 3. Configure Environment

Create a `.env` file in the root directory:

```env
# Network Configuration
SOLANA_NETWORK=devnet
SOLANA_RPC_URL=https://api.devnet.solana.com

# Program Configuration
PROGRAM_ID=YourProgramIDHere

# DEX Configuration
DEX1_PROGRAM_ID=RaydiumProgramID
DEX2_PROGRAM_ID=OrcaProgramID

# Treasury Configuration
TREASURY_WALLET=YourTreasuryWalletAddress

# Arbitrage Settings
MIN_PROFIT_THRESHOLD=100000000
MAX_TRADE_AMOUNT=10000000000

# Oracle Configuration
PYTH_PROGRAM_ID=PythProgramID
SWITCHBOARD_PROGRAM_ID=SwitchboardProgramID

# Monitoring
ENABLE_MONITORING=true
METRICS_PORT=9090
```

### 4. Set Up Solana Wallet

```bash
# Create a new wallet (or import existing)
solana-keygen new --outfile ~/.config/solana/id.json

# Check your wallet address
solana address

# Request airdrop on devnet (for testing)
solana airdrop 2
```

## 🏗️ Building the Project

### Build Smart Contracts

```bash
# Build Anchor program
anchor build

# Get the program ID
anchor keys list
```

### Update Program ID

After building, update the program ID in:
- `src/lib.rs` (line 5: `declare_id!`)
- `.env` file (if using environment-based configuration)

### Build the Solana Program

This repository contains only the on-chain Solana program (no standalone binary targets).

```bash
# Build the Solana program with Anchor
anchor build

# Or use cargo directly
cargo build-bpf
```

**Note**: This crate does not define a standalone Rust binary (no `src/main.rs` or `src/bin/*`),
so commands like `cargo run --release` or `./target/release/smsdao` are not available.

## 🧪 Testing

### Run Unit Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture
```

### Run Integration Tests

```bash
# Start local validator and run Anchor tests
anchor test

# Or manually with cargo
cargo test --test some-integration-tests
```

### Run Benchmarks

```bash
cargo bench
```

## 🚀 Using the Program

### Deploy to Devnet

```bash
# Deploy the program
anchor deploy --provider.cluster devnet

# Get the program ID
solana address -k target/deploy/smsdao-keypair.json
```

### Create an Off-Chain Bot (Separate Project)

To exercise the program, you'll need to create a separate off-chain client application:

1. Create a new Rust crate for your bot:
   ```bash
   cargo new smsdao-bot
   cd smsdao-bot
   ```

2. Add dependencies for Solana client interaction:
   ```toml
   [dependencies]
   anchor-client = "0.28"
   solana-sdk = "1.16"
   ```

3. Implement bot logic that calls the program's instructions.

4. Run your bot:
   ```bash
   cargo run --release -- \
     --network devnet \
     --min-profit 100000000 \
     --dex1 RaydiumProgramID \
     --dex2 OrcaProgramID
   ```

Refer to the [API Reference](API_REFERENCE.md) for instruction details.

## 📊 Monitoring

### View Logs

```bash
# View real-time logs
tail -f logs/smsdao.log

# With filtering
grep "arbitrage" logs/smsdao.log
```

### Check System Status

```bash
# Check bot status
curl http://localhost:9090/health

# View metrics
curl http://localhost:9090/metrics
```

## 🔍 Verifying Installation

Manually verify your installation:

```bash
# Check Rust version
rustc --version  # Should be 1.70.0 or higher

# Check Solana CLI
solana --version  # Should be 1.16.0 or higher

# Check Anchor
anchor --version  # Should be 0.28.0 or higher

# Check Node.js (optional, for client SDK)
node --version  # Should be 18.0.0 or higher
```

Expected output:
```
rustc 1.70.0 (or higher)
solana-cli 1.16.0 (or higher)
anchor-cli 0.28.0 (or higher)
✓ Node.js installed (version 18.0.0)
✓ Dependencies installed
✓ Configuration valid
✓ Wallet configured
✓ Network accessible

All checks passed! You're ready to use SMSDAO.
```

## 📚 Next Steps

Now that you have SMSDAO installed and running:

1. **Explore the Architecture**: Read the [Architecture Documentation](ARCHITECTURE.md) to understand how SMSDAO works
2. **Configure Your Setup**: Review the [Configuration Guide](CONFIGURATION.md) for advanced options
3. **Deploy to Production**: Follow the [Deployment Guide](DEPLOYMENT.md) when you're ready
4. **Enable Automation**: Set up [Auto Sync](AUTO_SYNC.md), [Auto Test](AUTO_TEST.md), [Auto Analysis](AUTO_ANALYSIS.md), and [Auto Fix](AUTO_FIX.md) features

## 🆘 Getting Help

If you encounter issues:
- Check the [Troubleshooting Guide](TROUBLESHOOTING.md)
- Search [existing issues](https://github.com/SMSDAO/SMSDAO/issues)
- Join our community discussions
- Create a new issue with detailed information

## 🎯 Quick Reference

Common commands:
```bash
# Build project
cargo build --release

# Run tests
cargo test

# Start bot
cargo run --release

# Deploy program
anchor deploy

# Check wallet balance
solana balance

# View program logs
solana logs
```

---

**Next**: [Architecture Overview](ARCHITECTURE.md) | [Configuration Guide](CONFIGURATION.md)
