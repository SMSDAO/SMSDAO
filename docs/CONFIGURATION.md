# Configuration Guide

Complete configuration reference for SMSDAO.

## 🎯 Overview

This guide covers all configuration options for SMSDAO, including environment variables, configuration files, and runtime parameters.

## 📁 Configuration Files

### Directory Structure

```
SMSDAO/
├── config/
│   ├── default.toml          # Default configuration
│   ├── development.toml      # Development overrides
│   ├── production.toml       # Production settings
│   ├── auto_sync.toml        # Auto Sync configuration
│   ├── auto_test.toml        # Auto Test configuration
│   ├── auto_analysis.toml    # Auto Analysis configuration
│   └── auto_fix.toml         # Auto Fix configuration
├── .env                      # Environment variables
└── Anchor.toml               # Anchor framework config
```

## ⚙️ Core Configuration

### `config/default.toml`

```toml
[general]
environment = "development"  # development, staging, production
log_level = "info"          # trace, debug, info, warn, error
log_file = "logs/smsdao.log"

[network]
solana_network = "devnet"   # devnet, testnet, mainnet-beta
solana_rpc_url = "https://api.devnet.solana.com"
commitment = "confirmed"     # processed, confirmed, finalized
connection_timeout_secs = 30
max_connections = 10

[network.fallback_rpcs]
enabled = true
endpoints = [
    "https://api.devnet.solana.com",
    "https://rpc.ankr.com/solana_devnet",
]

[program]
program_id = "YourProgramIDHere"
update_authority = "YourAuthorityPubkey"

[arbitrage]
enabled = true
min_profit_threshold = 100000000  # 0.1 SOL in lamports
max_trade_amount = 10000000000    # 10 SOL
max_slippage = 0.01               # 1%
execution_timeout_secs = 30

[arbitrage.dex]
dex1_program_id = "RaydiumProgramID"
dex2_program_id = "OrcaProgramID"
dex1_name = "Raydium"
dex2_name = "Orca"

[arbitrage.token_pairs]
pairs = [
    { token_a = "SOL", token_b = "USDC" },
    { token_a = "SOL", token_b = "USDT" },
]

[treasury]
treasury_wallet = "YourTreasuryWalletAddress"
token_a_vault = "TokenAVaultPubkey"
token_b_vault = "TokenBVaultPubkey"
fee_percentage = 0.003  # 0.3%

[governance]
proposal_threshold = 1000000000  # Min tokens to create proposal
voting_period_secs = 259200      # 3 days
timelock_period_secs = 86400     # 1 day
quorum_percentage = 0.1          # 10% of total supply

[oracle]
provider = "pyth"  # pyth, switchboard
pyth_program_id = "PythProgramID"
switchboard_program_id = "SwitchboardProgramID"
update_interval_secs = 5
max_price_age_secs = 60

[monitoring]
enabled = true
metrics_port = 9090
health_check_interval_secs = 30
alert_on_errors = true

[monitoring.prometheus]
enabled = true
endpoint = "/metrics"

[monitoring.alerts]
slack_webhook = "https://hooks.slack.com/..."
email_recipients = ["admin@smsdao.io"]
```

### `config/production.toml`

Production-specific overrides:

```toml
[general]
environment = "production"
log_level = "warn"

[network]
solana_network = "mainnet-beta"
solana_rpc_url = "https://api.mainnet-beta.solana.com"
commitment = "finalized"

[network.fallback_rpcs]
endpoints = [
    "https://api.mainnet-beta.solana.com",
    "https://solana-api.projectserum.com",
    "https://rpc.ankr.com/solana",
]

[arbitrage]
min_profit_threshold = 500000000  # Higher threshold for mainnet
max_slippage = 0.005              # Tighter slippage

[monitoring]
alert_on_errors = true
```

## 🌐 Environment Variables

### `.env` File

```env
# Network Configuration
SOLANA_NETWORK=devnet
SOLANA_RPC_URL=https://api.devnet.solana.com
SOLANA_COMMITMENT=confirmed

# Program Configuration
PROGRAM_ID=YourProgramIDHere
PROGRAM_AUTHORITY=YourAuthorityKeypairPath

# Wallet Configuration
WALLET_PATH=~/.config/solana/id.json
TREASURY_WALLET=YourTreasuryWalletAddress

# DEX Configuration
DEX1_PROGRAM_ID=RaydiumProgramID
DEX2_PROGRAM_ID=OrcaProgramID

# Arbitrage Settings
MIN_PROFIT_THRESHOLD=100000000
MAX_TRADE_AMOUNT=10000000000
MAX_SLIPPAGE=0.01

# Oracle Configuration
ORACLE_PROVIDER=pyth
PYTH_PROGRAM_ID=PythProgramID
SWITCHBOARD_PROGRAM_ID=SwitchboardProgramID

# Treasury Configuration
TOKEN_A_VAULT=TokenAVaultPubkey
TOKEN_B_VAULT=TokenBVaultPubkey

# Monitoring
ENABLE_MONITORING=true
METRICS_PORT=9090
LOG_LEVEL=info

# Auto Features
ENABLE_AUTO_SYNC=true
ENABLE_AUTO_TEST=false
ENABLE_AUTO_ANALYSIS=true
ENABLE_AUTO_FIX=true

# Notifications
SLACK_WEBHOOK_URL=https://hooks.slack.com/...
EMAIL_SMTP_SERVER=smtp.gmail.com
EMAIL_FROM=noreply@smsdao.io
EMAIL_TO=admin@smsdao.io

# Security
REQUIRE_2FA=true
API_KEY=YourSecureAPIKey
ENCRYPT_LOGS=false
```

## 🔧 Command Line Arguments

Override configuration via CLI:

```bash
# Basic usage
cargo run --release -- \
  --config config/production.toml \
  --network mainnet-beta \
  --log-level warn

# Arbitrage settings
cargo run --release -- \
  --min-profit 500000000 \
  --max-trade-amount 5000000000 \
  --max-slippage 0.005

# Enable features
cargo run --release -- \
  --enable-auto-sync \
  --enable-auto-analysis \
  --enable-auto-fix

# Monitoring
cargo run --release -- \
  --metrics-port 9090 \
  --enable-monitoring

# Full example
cargo run --release -- \
  --config config/production.toml \
  --network mainnet-beta \
  --min-profit 500000000 \
  --enable-auto-sync \
  --enable-auto-analysis \
  --enable-auto-fix \
  --log-level warn
```

## 🔒 Security Configuration

### Access Control

```toml
[security]
enabled = true
require_authentication = true
api_key_header = "X-API-Key"
rate_limit_per_minute = 100

[security.roles]
# Define role-based access
[security.roles.owner]
permissions = ["*"]
addresses = ["OwnerPubkey1", "OwnerPubkey2"]

[security.roles.admin]
permissions = ["read", "execute", "configure"]
addresses = ["AdminPubkey1"]

[security.roles.operator]
permissions = ["read", "execute"]
addresses = ["OperatorPubkey1", "OperatorPubkey2"]

[security.roles.viewer]
permissions = ["read"]
addresses = ["ViewerPubkey1"]

[security.firewall]
enabled = true
whitelist_ips = ["192.168.1.0/24", "10.0.0.0/8"]
blacklist_ips = []

[security.audit]
enabled = true
log_all_actions = true
audit_log_file = "logs/audit.log"
retention_days = 90
```

### Encryption

```toml
[encryption]
enabled = true
algorithm = "AES-256-GCM"
key_rotation_days = 30
encrypt_logs = true
encrypt_config = false
```

## 📊 Monitoring Configuration

### Metrics & Alerts

```toml
[monitoring.metrics]
enabled = true
port = 9090
path = "/metrics"
collection_interval_secs = 10

[monitoring.metrics.exporters]
prometheus = true
graphite = false
datadog = false

[monitoring.health_checks]
enabled = true
interval_secs = 30
timeout_secs = 5
endpoints = [
    { name = "solana_rpc", url = "${SOLANA_RPC_URL}/health" },
    { name = "database", type = "database" },
    { name = "arbitrage_bot", type = "internal" },
]

[monitoring.alerts]
enabled = true

[monitoring.alerts.conditions]
# Alert on high error rate
[[monitoring.alerts.conditions.rules]]
name = "high_error_rate"
metric = "error_rate"
threshold = 0.05  # 5%
duration_secs = 300
severity = "critical"

# Alert on low success rate
[[monitoring.alerts.conditions.rules]]
name = "low_success_rate"
metric = "success_rate"
threshold = 0.95
comparator = "less_than"
duration_secs = 600
severity = "warning"

# Alert on high latency
[[monitoring.alerts.conditions.rules]]
name = "high_latency"
metric = "avg_execution_time_ms"
threshold = 5000
duration_secs = 300
severity = "warning"
```

## 🔄 Auto Features Configuration

See dedicated documentation for each:
- [Auto Sync Configuration](AUTO_SYNC.md#configuration)
- [Auto Test Configuration](AUTO_TEST.md#configuration)
- [Auto Analysis Configuration](AUTO_ANALYSIS.md#configuration)
- [Auto Fix Configuration](AUTO_FIX.md#configuration)

## 🗄️ Database Configuration

```toml
[database]
enabled = true
type = "postgresql"  # postgresql, mongodb, sqlite
host = "localhost"
port = 5432
database = "smsdao"
username = "smsdao_user"
password = "${DB_PASSWORD}"  # From environment
max_connections = 10
connection_timeout_secs = 30

[database.migrations]
auto_migrate = true
migrations_path = "migrations/"

[database.backup]
enabled = true
interval_hours = 24
retention_days = 30
backup_path = "backups/"
```

## 🌉 Bridge Configuration

For multi-chain operations:

```toml
[bridge]
enabled = true
provider = "wormhole"

[bridge.wormhole]
solana_bridge_address = "WormholeBridgeAddress"
base_bridge_address = "0x123...def"
guardian_rpc = "https://wormhole-v2-mainnet-api.certus.one"

[bridge.chains]
[bridge.chains.solana]
enabled = true
network = "mainnet-beta"

[bridge.chains.base]
enabled = true
rpc_url = "https://mainnet.base.org"
chain_id = 8453
```

## 📝 Logging Configuration

```toml
[logging]
level = "info"  # trace, debug, info, warn, error
format = "json"  # json, pretty, compact
output = "file"  # file, stdout, both

[logging.file]
path = "logs/smsdao.log"
max_size_mb = 100
max_backups = 10
max_age_days = 30
compress = true

[logging.filters]
# Filter sensitive data
mask_secrets = true
mask_private_keys = true
mask_wallet_addresses = false

[logging.targets]
# Different log levels for different modules
default = "info"
"smsdao::arbitrage" = "debug"
"smsdao::treasury" = "info"
"smsdao::governance" = "info"
```

## 🧪 Testing Configuration

```toml
[testing]
enabled = true
use_test_validator = true
test_validator_port = 8899

[testing.accounts]
# Pre-funded test accounts
test_wallet = "TestWalletKeypairPath"
test_token_a = "TestTokenAMint"
test_token_b = "TestTokenBMint"

[testing.mocks]
# Mock external services
mock_dex_prices = true
mock_oracle_feeds = true
mock_network_delays = false
```

## 🔧 Configuration Priority

Configuration is loaded in this order (later overrides earlier):

1. Default configuration (`config/default.toml`)
2. Environment-specific config (`config/{environment}.toml`)
3. Environment variables (`.env`)
4. Command line arguments
5. Runtime configuration changes

## ✅ Configuration Validation

Validate your configuration:

```bash
# Validate configuration files
cargo run --bin validate-config -- --config config/production.toml

# Test configuration
cargo run --release -- --config config/production.toml --dry-run

# Show effective configuration
cargo run --release -- --show-config
```

## 📚 Configuration Best Practices

1. **Environment Variables for Secrets**: Never commit secrets to configuration files
2. **Environment-Specific Configs**: Maintain separate configs for dev/staging/prod
3. **Validation**: Always validate configuration before deployment
4. **Documentation**: Document all configuration options
5. **Defaults**: Provide sensible defaults for all options
6. **Versioning**: Version control configuration files
7. **Backup**: Keep backups of production configurations
8. **Review**: Regular security review of configurations

## 🔍 Troubleshooting Configuration

### Common Issues

**Configuration not loading:**
```bash
# Check file exists
ls -la config/production.toml

# Validate TOML syntax
cargo run --bin validate-config

# Check environment variables
env | grep SMSDAO
```

**Invalid values:**
```bash
# View effective configuration
cargo run --release -- --show-config

# Test with verbose logging
RUST_LOG=debug cargo run --release
```

**Permission errors:**
```bash
# Check file permissions
ls -la config/
chmod 600 config/production.toml
```

---

**Related Documentation:**
- [Getting Started](GETTING_STARTED.md)
- [Deployment Guide](DEPLOYMENT.md)
- [Security Documentation](SECURITY.md)
