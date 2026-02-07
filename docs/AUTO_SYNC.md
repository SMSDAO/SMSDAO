# Auto Sync

Automated synchronization across chains, DEXs, and data sources to ensure SMSDAO operates with consistent, up-to-date information.

## 🎯 Overview

Auto Sync is a critical automation feature that maintains data consistency across:
- Multiple blockchain networks (Solana, Base, L3s)
- DEX price feeds
- Oracle data sources
- Treasury balances
- Governance states

## 🔄 Sync Components

### 1. Price Feed Synchronization

**Purpose**: Keep DEX prices synchronized across all monitored exchanges.

**Configuration** (`config/auto_sync.toml`):
```toml
[price_sync]
enabled = true
interval_ms = 1000           # Sync every 1 second
sources = ["raydium", "orca", "jupiter"]
retry_attempts = 3
timeout_ms = 5000

[price_sync.cache]
enabled = true
ttl_seconds = 5
max_entries = 1000
```

**Features:**
- Real-time price monitoring
- Automatic retry on failure
- Price deviation alerts
- Historical data archival

**Implementation:**
```rust
pub struct PriceSyncManager {
    sources: Vec<PriceSource>,
    cache: Arc<Mutex<PriceCache>>,
    config: PriceSyncConfig,
}

impl PriceSyncManager {
    pub async fn sync_prices(&self) -> Result<()> {
        for source in &self.sources {
            match source.fetch_prices().await {
                Ok(prices) => {
                    self.cache.lock().await.update(prices);
                    emit_event!(PricesSynced { source: source.name() });
                }
                Err(e) => {
                    log::warn!("Price sync failed for {}: {}", source.name(), e);
                    self.retry_with_backoff(source).await?;
                }
            }
        }
        Ok(())
    }
}
```

### 2. Multi-Chain State Synchronization

**Purpose**: Maintain consistent state across different blockchain networks.

**Configuration:**
```toml
[chain_sync]
enabled = true
chains = ["solana", "base"]

[chain_sync.solana]
rpc_url = "https://api.mainnet-beta.solana.com"
commitment = "confirmed"
sync_interval_ms = 2000

[chain_sync.base]
rpc_url = "https://mainnet.base.org"
sync_interval_ms = 3000
```

**Sync Operations:**
- Account balance synchronization
- Transaction status updates
- Smart contract state verification
- Event log processing

**Example:**
```typescript
const syncManager = new MultiChainSyncManager({
  chains: ['solana', 'base'],
  syncInterval: 2000,
});

syncManager.on('stateChanged', (chain, state) => {
  console.log(`${chain} state updated:`, state);
});

await syncManager.start();
```

### 3. Treasury Balance Synchronization

**Purpose**: Track treasury balances across all chains and tokens.

**Features:**
- Real-time balance tracking
- Multi-chain aggregation
- Historical balance snapshots
- Anomaly detection

**Configuration:**
```toml
[treasury_sync]
enabled = true
interval_ms = 5000
wallets = [
    { chain = "solana", address = "7yKZw...Abc" },
    { chain = "base", address = "0x123...def" },
]

[treasury_sync.alerts]
large_withdrawal_threshold = 10000  # USD
balance_drop_threshold = 0.1         # 10% drop
```

**Implementation:**
```rust
pub async fn sync_treasury_balances(&self) -> Result<TreasurySnapshot> {
    let mut snapshot = TreasurySnapshot::new();
    
    for wallet in &self.config.wallets {
        let balance = self.fetch_balance(wallet).await?;
        
        // Check for anomalies
        if let Some(prev_balance) = self.get_previous_balance(wallet) {
            let drop_percentage = (prev_balance - balance) / prev_balance;
            if drop_percentage > self.config.alerts.balance_drop_threshold {
                self.alert_balance_drop(wallet, drop_percentage).await?;
            }
        }
        
        snapshot.add_balance(wallet, balance);
    }
    
    self.store_snapshot(snapshot.clone()).await?;
    Ok(snapshot)
}
```

### 4. Governance State Synchronization

**Purpose**: Keep governance proposals and votes synchronized.

**Features:**
- Proposal status updates
- Vote tallying
- Execution readiness checks
- Historical governance data

**Configuration:**
```toml
[governance_sync]
enabled = true
interval_ms = 10000
include_historical = true
max_historical_proposals = 100
```

## 🚀 Usage

### Enable Auto Sync

```bash
# Enable all sync features
cargo run --release -- --enable-auto-sync

# Enable specific sync components
cargo run --release -- \
  --enable-price-sync \
  --enable-chain-sync \
  --enable-treasury-sync
```

### Configuration File

Create `config/auto_sync.toml`:
```toml
[general]
enabled = true
log_level = "info"

[price_sync]
enabled = true
interval_ms = 1000

[chain_sync]
enabled = true
interval_ms = 2000

[treasury_sync]
enabled = true
interval_ms = 5000

[governance_sync]
enabled = true
interval_ms = 10000
```

### Programmatic Usage

**Rust:**
```rust
use smsdao::sync::{AutoSyncManager, SyncConfig};

#[tokio::main]
async fn main() -> Result<()> {
    let config = SyncConfig::from_file("config/auto_sync.toml")?;
    let sync_manager = AutoSyncManager::new(config);
    
    // Start all sync tasks
    sync_manager.start_all().await?;
    
    // Monitor sync status
    loop {
        let status = sync_manager.get_status().await;
        println!("Sync status: {:?}", status);
        tokio::time::sleep(Duration::from_secs(10)).await;
    }
}
```

**TypeScript:**
```typescript
import { AutoSyncManager } from '@smsdao/sdk';

const syncManager = new AutoSyncManager({
  priceSync: { enabled: true, intervalMs: 1000 },
  chainSync: { enabled: true, intervalMs: 2000 },
  treasurySync: { enabled: true, intervalMs: 5000 },
});

await syncManager.start();

// Listen for sync events
syncManager.on('pricesSynced', (data) => {
  console.log('Prices updated:', data);
});

syncManager.on('syncError', (error) => {
  console.error('Sync error:', error);
});
```

## 📊 Monitoring

### Sync Metrics

Monitor sync performance via the `/metrics` endpoint:

```json
{
  "auto_sync": {
    "price_sync": {
      "enabled": true,
      "last_sync": 1707339515,
      "sync_count": 1000,
      "error_count": 5,
      "avg_duration_ms": 150
    },
    "chain_sync": {
      "enabled": true,
      "chains_synced": 2,
      "last_sync": 1707339516,
      "sync_count": 500,
      "error_count": 2
    },
    "treasury_sync": {
      "enabled": true,
      "wallets_tracked": 5,
      "total_balance_usd": 150500.0,
      "last_sync": 1707339517
    }
  }
}
```

### Sync Logs

View sync activity in logs:
```bash
tail -f logs/auto_sync.log
```

Example log output:
```
[2026-02-07 20:30:15] INFO  Price sync completed - sources: 3, duration: 145ms
[2026-02-07 20:30:17] INFO  Chain sync completed - chains: 2, duration: 230ms
[2026-02-07 20:30:20] INFO  Treasury sync completed - wallets: 5, total: $150,500
[2026-02-07 20:30:25] WARN  Price sync retry - source: Raydium, attempt: 1/3
[2026-02-07 20:30:30] INFO  Governance sync completed - proposals: 5
```

## ⚙️ Advanced Configuration

### Custom Sync Intervals

```toml
[sync_intervals]
# Fast sync for critical data
price_sync_ms = 500

# Medium sync for chain data
chain_sync_ms = 2000

# Slow sync for less critical data
treasury_sync_ms = 10000
governance_sync_ms = 30000
```

### Retry Configuration

```toml
[retry]
max_attempts = 3
initial_delay_ms = 1000
max_delay_ms = 10000
backoff_multiplier = 2.0
```

### Alert Configuration

```toml
[alerts]
enabled = true
channels = ["email", "slack", "webhook"]

[alerts.thresholds]
price_deviation_percent = 5.0
sync_failure_count = 3
sync_delay_seconds = 60
```

## 🔧 Troubleshooting

### Sync Not Working

**Check sync status:**
```bash
curl http://localhost:9090/sync/status
```

**Common issues:**
1. Network connectivity problems
2. Invalid RPC endpoints
3. Rate limiting
4. Insufficient permissions

### High Sync Latency

**Causes:**
- Slow RPC endpoints
- Network congestion
- Too many sync sources

**Solutions:**
- Use dedicated RPC nodes
- Adjust sync intervals
- Implement caching
- Use multiple fallback RPCs

### Sync Data Inconsistency

**Diagnosis:**
```bash
# Check sync logs
grep "ERROR" logs/auto_sync.log

# Verify data sources
curl http://localhost:9090/sync/sources
```

**Resolution:**
1. Force full re-sync
2. Clear cache
3. Verify RPC endpoint health

## 🔐 Security Considerations

1. **RPC Authentication**: Use authenticated RPC endpoints
2. **Data Validation**: Validate all synced data before use
3. **Rate Limiting**: Respect API rate limits
4. **Fallback Sources**: Configure multiple data sources
5. **Monitoring**: Monitor for data manipulation attempts

## 📈 Performance Optimization

**Best Practices:**
1. Use appropriate sync intervals for each component
2. Enable caching for frequently accessed data
3. Batch operations when possible
4. Use connection pooling
5. Monitor and adjust based on metrics

**Benchmarks:**
- Price sync: <200ms for 3 sources
- Chain sync: <500ms for 2 chains
- Treasury sync: <1s for 10 wallets
- Governance sync: <2s for 100 proposals

---

**Related Documentation:**
- [Auto Test](AUTO_TEST.md)
- [Auto Analysis](AUTO_ANALYSIS.md)
- [Auto Fix](AUTO_FIX.md)
- [Architecture](ARCHITECTURE.md)
