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
- `Anchor.toml`
- `src/main.rs` (line 5: `declare_id!`)
- `.env` file

### Compile Rust Binaries

```bash
# Build all binaries
cargo build --release

# Build specific binary
cargo build --bin another_executable --release
```

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
# Start local validator
solana-test-validator

# In another terminal, run integration tests
anchor test
```

### Run Benchmarks

```bash
cargo bench
```

## 🚀 Running the Arbitrage Bot

### Local Development

```bash
# Start the arbitrage bot
cargo run --release

# Or use the binary directly
./target/release/smsdao
```

### With Custom Configuration

```bash
cargo run --release -- \
  --network devnet \
  --min-profit 100000000 \
  --dex1 RaydiumProgramID \
  --dex2 OrcaProgramID
```

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

Run the verification script:

```bash
# Check all prerequisites
./scripts/verify-installation.sh
```

Expected output:
```
✓ Rust installed (version 1.70.0)
✓ Solana CLI installed (version 1.16.0)
✓ Anchor installed (version 0.28.0)
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
