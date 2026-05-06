# Troubleshooting Guide

Common issues and solutions for SMSDAO.

## 🎯 Overview

This guide helps you diagnose and resolve common issues with SMSDAO. For each issue, we provide:
- Symptoms
- Possible causes
- Solutions
- Prevention tips

## 📋 Table of Contents

- [Installation Issues](#installation-issues)
- [Configuration Issues](#configuration-issues)
- [Smart Contract Issues](#smart-contract-issues)
- [Runtime Issues](#runtime-issues)
- [Performance Issues](#performance-issues)
- [Network Issues](#network-issues)
- [FAQ](#faq)
- [Error Codes](#error-codes)

## 🔧 Installation Issues

### Issue: Rust Installation Fails

**Symptoms:**
```bash
error: command failed: 'rustc'
```

**Solution:**
```bash
# Uninstall existing Rust
rustup self uninstall

# Reinstall Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version
cargo --version
```

### Issue: Solana CLI Not Found

**Symptoms:**
```bash
solana: command not found
```

**Solution:**
```bash
# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# Add to PATH (add to ~/.bashrc or ~/.zshrc)
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"

# Reload shell
source ~/.bashrc

# Verify
solana --version
```

### Issue: Anchor Build Fails

**Symptoms:**
```bash
error: failed to compile program
```

**Solutions:**

1. **Update Anchor:**
```bash
avm install latest
avm use latest
```

2. **Clean and rebuild:**
```bash
anchor clean
cargo clean
anchor build
```

3. **Check dependencies:**
```bash
cargo update
```

## ⚙️ Configuration Issues

### Issue: Invalid Configuration File

**Symptoms:**
```bash
Error: Failed to parse configuration
```

**Solutions:**

1. **Validate TOML syntax:**
```bash
# Check for syntax errors using a TOML linter
# Install taplo: cargo install taplo-cli
taplo lint config/production.toml

# Or use Python toml-cli
# Install: pip install toml-cli
toml-cli lint config/production.toml
```

2. **Common mistakes:**
```toml
# ❌ Bad: Missing quotes
solana_rpc_url = https://api.devnet.solana.com

# ✅ Good: With quotes
solana_rpc_url = "https://api.devnet.solana.com"

# ❌ Bad: Wrong data type
min_profit_threshold = "100000000"

# ✅ Good: Correct type
min_profit_threshold = 100000000
```

### Issue: Environment Variables Not Loaded

**Symptoms:**
```bash
Error: Environment variable PROGRAM_ID not found
```

**Solutions:**

1. **Check .env file exists:**
```bash
ls -la .env
```

2. **Load environment variables:**
```bash
# Load manually
source .env

# Or use with cargo
cargo run --release
```

3. **Verify variables:**
```bash
echo $PROGRAM_ID
echo $SOLANA_RPC_URL
```

### Issue: Wrong Network Configuration

**Symptoms:**
```bash
Error: Invalid network configuration
Transaction sent to wrong network
```

**Solution:**
```bash
# Check current network
solana config get

# Set correct network
solana config set --url mainnet-beta
# or
solana config set --url devnet

# Verify
solana config get
```

## 📜 Smart Contract Issues

### Issue: Program Deployment Fails

**Symptoms:**
```bash
Error: Failed to deploy program
Insufficient funds
```

**Solutions:**

1. **Check balance:**
```bash
solana balance

# Need funds for deployment
# Devnet: Request airdrop
solana airdrop 2

# Mainnet: Transfer SOL to wallet
```

2. **Check program size:**
```bash
# View program size
ls -lh target/deploy/*.so

# If too large, optimize
cargo build-bpf --release
```

3. **Verify program ID:**
```bash
# Get program ID
anchor keys list

# Update in code
# Edit src/main.rs: declare_id!("YourProgramID")
# Edit Anchor.toml: [programs.devnet] smsdao = "YourProgramID"
```

### Issue: Transaction Fails

**Symptoms:**
```bash
Error: Transaction simulation failed
Error: custom program error: 0x1
```

**Solutions:**

1. **Check logs:**
```bash
# View program logs
solana logs

# Or specific program
solana logs | grep YOUR_PROGRAM_ID
```

2. **Decode error code:**
```rust
// Error code 0x1 = 1 = NotProfitable
pub enum ArbitrageError {
    #[msg("Arbitrage is not profitable")]
    NotProfitable = 6000,  // Anchor adds 6000 offset
}
// So error 6000 = 0x1770 in hex
```

3. **Common errors:**
```
0x0 (0): Unauthorized - Check signer permissions
0x1 (1): NotProfitable - Increase amount or wait for better prices
0x2 (2): InsufficientBalance - Add funds to vault
0x3 (3): SlippageTooHigh - Increase slippage tolerance
```

### Issue: Account Not Found

**Symptoms:**
```bash
Error: Account not found: ADDRESS
```

**Solutions:**

1. **Check account exists:**
```bash
solana account ADDRESS
```

2. **Initialize account if needed:**
```bash
# Run initialization script
anchor run initialize
```

3. **Verify account ownership:**
```bash
# Check account owner
solana account ADDRESS --output json | jq '.owner'
```

## 🏃 Runtime Issues

### Issue: Bot Not Executing Trades

**Symptoms:**
- Bot running but no trades executed
- Logs show "No profitable opportunities"

**Solutions:**

1. **Check price feeds:**
```bash
# Test DEX connectivity
curl http://localhost:9090/dex/prices
```

2. **Lower profit threshold:**
```toml
# In config file
min_profit_threshold = 50000000  # Lower threshold
```

3. **Check market conditions:**
```bash
# View current opportunities
curl http://localhost:9090/arbitrage/opportunities
```

4. **Verify DEX configuration:**
```bash
# Check DEX programs are correct
echo $DEX1_PROGRAM_ID
echo $DEX2_PROGRAM_ID
```

### Issue: High Error Rate

**Symptoms:**
```bash
WARN: Transaction failed: Network timeout
ERROR: Execution error rate: 15%
```

**Solutions:**

1. **Check RPC health:**
```bash
# Test RPC endpoint
curl https://api.devnet.solana.com/health

# Switch to backup RPC
export SOLANA_RPC_URL="https://rpc.ankr.com/solana_devnet"
```

2. **Adjust timeouts:**
```toml
[arbitrage]
execution_timeout_secs = 60  # Increase timeout
```

3. **Enable retry mechanism:**
```toml
[auto_fix]
enabled = true
max_retry_attempts = 3
```

### Issue: Memory Usage High

**Symptoms:**
```bash
System using >2GB memory
Process killed by OOM killer
```

**Solutions:**

1. **Check for memory leaks:**
```bash
# Monitor memory
top -p $(pidof smsdao)

# Or use htop
htop
```

2. **Optimize cache:**
```toml
[price_sync.cache]
max_entries = 500  # Reduce cache size
ttl_seconds = 3    # Shorter TTL
```

3. **Restart periodically:**
```bash
# Add to cron for daily restart
0 0 * * * systemctl restart smsdao
```

## 🚀 Performance Issues

### Issue: Slow Execution Times

**Symptoms:**
```bash
Average execution time: 5000ms
Timeout errors increasing
```

**Solutions:**

1. **Use dedicated RPC:**
```bash
# Switch to paid RPC service
# Helius, Triton, QuickNode
export SOLANA_RPC_URL="https://mainnet.helius-rpc.com/?api-key=YOUR_KEY"
```

2. **Optimize parameters:**
```toml
[arbitrage]
max_trade_amount = 5000000000  # Reduce trade size
max_slippage = 0.02            # Increase slippage tolerance
```

3. **Enable parallel processing:**
```toml
[general]
parallel_execution = true
max_parallel_jobs = 4
```

### Issue: High Gas Costs

**Symptoms:**
```bash
Gas costs exceeding profits
Average gas: 0.05 SOL per trade
```

**Solutions:**

1. **Optimize compute units:**
```rust
// Request specific compute units
let ix = ComputeBudgetInstruction::set_compute_unit_limit(200_000);
```

2. **Increase profit threshold:**
```toml
[arbitrage]
min_profit_threshold = 200000000  # 0.2 SOL
```

3. **Batch transactions:**
```rust
// Combine multiple operations in one transaction
let tx = Transaction::new_signed_with_payer(
    &[ix1, ix2, ix3],
    Some(&payer.pubkey()),
    &[&payer],
    recent_blockhash,
);
```

## 🌐 Network Issues

### Issue: RPC Connection Failures

**Symptoms:**
```bash
Error: Connection timeout
Error: Rate limit exceeded
```

**Solutions:**

1. **Configure fallback RPCs:**
```toml
[network.fallback_rpcs]
enabled = true
endpoints = [
    "https://api.mainnet-beta.solana.com",
    "https://solana-api.projectserum.com",
    "https://rpc.ankr.com/solana",
]
```

2. **Implement exponential backoff:**
```toml
[retry]
max_attempts = 5
initial_delay_ms = 1000
max_delay_ms = 30000
exponential_backoff = true
```

3. **Use connection pooling:**
```toml
[network]
max_connections = 10
connection_timeout_secs = 30
```

### Issue: Transaction Confirmation Delays

**Symptoms:**
```bash
Transaction pending for >60 seconds
Transactions timing out
```

**Solutions:**

1. **Adjust commitment level:**
```toml
[network]
commitment = "confirmed"  # Instead of "finalized"
```

2. **Increase priority fees:**
```rust
let priority_fee = ComputeBudgetInstruction::set_compute_unit_price(
    10_000  // Micro-lamports
);
```

3. **Monitor network congestion:**
```bash
# Check network performance
solana block-production

# Check TPS
solana transaction-count
```

## ❓ FAQ

### How do I update SMSDAO?

```bash
# Pull latest changes
git pull origin main

# Rebuild
cargo build --release

# Restart service
sudo systemctl restart smsdao
```

### How do I check if the bot is working?

```bash
# Check service status
sudo systemctl status smsdao

# Check logs
tail -f /var/log/smsdao/smsdao.log

# Check metrics
curl http://localhost:9090/metrics

# Check health
curl http://localhost:9090/health
```

### How do I increase profitability?

1. Lower minimum profit threshold (but not too low)
2. Add more DEX integrations
3. Optimize gas usage
4. Use dedicated RPC endpoints
5. Increase trade frequency

### How do I handle failed transactions?

Failed transactions are normal. The bot should:
1. Log the error
2. Retry if appropriate
3. Continue monitoring for new opportunities

Enable Auto Fix for automatic retry:
```toml
[auto_fix]
enabled = true
```

### Where are my funds?

Check wallet balances:
```bash
# Main wallet
solana balance

# Token accounts
spl-token accounts

# Program accounts
solana account YOUR_STATE_ACCOUNT
```

## 🔢 Error Codes

### Program Error Codes

| Code | Hex | Name | Description | Solution |
|------|-----|------|-------------|----------|
| 6000 | 0x1770 | NotProfitable | Trade not profitable | Lower threshold or wait |
| 6001 | 0x1771 | InsufficientBalance | Not enough funds | Add funds to vault |
| 6002 | 0x1772 | SlippageTooHigh | Slippage exceeded | Increase slippage tolerance |
| 6003 | 0x1773 | Unauthorized | Wrong signer | Check wallet permissions |
| 6004 | 0x1774 | InvalidAmount | Invalid trade amount | Check amount constraints |
| 6005 | 0x1775 | InvalidDexProgram | Wrong DEX program | Verify DEX program IDs |
| 6006 | 0x1776 | SystemPaused | Emergency pause active | Wait for unpause |

### System Error Codes

| Code | Name | Description | Solution |
|------|------|-------------|----------|
| 1001 | ConfigError | Configuration error | Check config file |
| 1002 | NetworkError | Network connection error | Check RPC endpoint |
| 1003 | DatabaseError | Database error | Check database connection |
| 1004 | AuthError | Authentication error | Check API keys |

## 🆘 Getting More Help

If you can't resolve your issue:

1. **Check Documentation**
   - [Getting Started](GETTING_STARTED.md)
   - [Configuration](CONFIGURATION.md)
   - [API Reference](API_REFERENCE.md)

2. **Search Issues**
   - [GitHub Issues](https://github.com/SMSDAO/SMSDAO/issues)
   - Search for similar problems

3. **Create Issue**
   - Provide error messages
   - Include logs
   - Describe steps to reproduce
   - Include environment details

4. **Community Help**
   - Join Discord
   - Ask in Discussions
   - Check FAQ

## 📊 Diagnostic Commands

Collect diagnostic information:

```bash
# System information
cargo --version
rustc --version
solana --version
anchor --version

# Configuration
cargo run --release -- --show-config

# Service status
sudo systemctl status smsdao

# Recent logs
tail -n 100 /var/log/smsdao/smsdao.log

# Error logs
grep ERROR /var/log/smsdao/smsdao.log

# Network connectivity
curl -I https://api.mainnet-beta.solana.com

# Account info
solana account YOUR_STATE_ACCOUNT
```

## 🔍 Debug Mode

Enable debug logging:

```bash
# Run with debug logging
RUST_LOG=debug cargo run --release

# Or set in config
[logging]
level = "debug"
```

---

**Still having issues?**
- Email: support@smsdao.io
- Discord: [Join Server](#)
- GitHub: [Open Issue](https://github.com/SMSDAO/SMSDAO/issues/new)

**Related Documentation:**
- [Getting Started](GETTING_STARTED.md)
- [Configuration](CONFIGURATION.md)
- [Security](SECURITY.md)
