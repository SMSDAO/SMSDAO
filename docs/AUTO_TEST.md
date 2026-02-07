# Auto Test

Automated testing framework for continuous validation of SMSDAO components, smart contracts, and integrations.

## 🎯 Overview

Auto Test provides comprehensive automated testing capabilities:
- Continuous integration testing
- Smart contract validation
- Integration testing across chains
- Performance benchmarking
- Regression testing
- Security testing

## 🧪 Test Framework

### Test Levels

1. **Unit Tests**: Individual component testing
2. **Integration Tests**: Multi-component interaction testing
3. **End-to-End Tests**: Full system workflow testing
4. **Performance Tests**: Load and stress testing
5. **Security Tests**: Vulnerability and exploit testing

## 🔧 Configuration

### Test Configuration File

Create `config/auto_test.toml`:

```toml
[general]
enabled = true
run_on_commit = true
run_on_schedule = true
schedule_cron = "0 */6 * * *"  # Every 6 hours
parallel_execution = true
max_parallel_jobs = 4

[unit_tests]
enabled = true
timeout_seconds = 300
coverage_threshold = 80
fail_on_coverage_drop = true

[integration_tests]
enabled = true
timeout_seconds = 600
test_networks = ["devnet", "testnet"]
cleanup_after_test = true

[e2e_tests]
enabled = true
timeout_seconds = 1800
scenarios = [
    "full_arbitrage_cycle",
    "governance_proposal_flow",
    "treasury_management",
    "multi_chain_operations"
]

[performance_tests]
enabled = true
benchmarks = ["arbitrage_execution", "price_feed_sync", "transaction_throughput"]
regression_threshold = 0.1  # 10% regression allowed

[security_tests]
enabled = true
vulnerability_scans = true
fuzzing_enabled = true
fuzzing_duration_minutes = 30
```

## 🚀 Running Tests

### Command Line

```bash
# Run all tests
cargo test

# Run specific test suite
cargo test --test integration_tests

# Run with auto-test features
cargo run --release -- --auto-test

# Run specific test type
cargo test unit::
cargo test integration::
cargo test e2e::

# Run tests with coverage
cargo tarpaulin --out Html --output-dir coverage/

# Run benchmarks
cargo bench
```

### Automated Execution

**On Git Commit**:
```yaml
# .github/workflows/auto-test.yml
name: Auto Test

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Run Auto Tests
        run: |
          cargo test --all-features
          cargo bench
```

**Scheduled Testing**:
```bash
# Add to crontab
0 */6 * * * cd /path/to/SMSDAO && cargo test --release
```

## 📋 Test Suites

### 1. Unit Tests

Test individual functions and components.

**Example: Arbitrage Calculation Test**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arbitrage_profit_calculation() {
        let dex1_price = 100_000_000; // 100 USDC
        let dex2_price = 101_000_000; // 101 USDC
        let amount = 1_000_000_000;   // 1 SOL
        
        let profit = calculate_profit(dex1_price, dex2_price, amount);
        
        assert!(profit > 0);
        assert_eq!(profit, 1_000_000_000); // Expected profit minus fees
    }

    #[test]
    fn test_min_profit_threshold() {
        let state = ArbitrageState {
            min_profit: 100_000_000,
            ..Default::default()
        };
        
        let small_profit = 50_000_000;
        assert!(!is_profitable(&state, small_profit));
        
        let large_profit = 150_000_000;
        assert!(is_profitable(&state, large_profit));
    }

    #[test]
    fn test_price_deviation_detection() {
        let prices = vec![100, 101, 102, 150]; // Last price is anomaly
        let avg = calculate_average(&prices[..3]);
        let deviation = (prices[3] - avg) as f64 / avg as f64;
        
        assert!(deviation > 0.1); // More than 10% deviation
    }
}
```

### 2. Integration Tests

Test interactions between components.

**Example: DEX Integration Test**

```rust
#[tokio::test]
async fn test_dex_price_fetch() {
    let config = TestConfig::default();
    let dex_client = DexClient::new(config);
    
    // Test Raydium integration
    let raydium_price = dex_client
        .fetch_price("raydium", "SOL/USDC")
        .await
        .expect("Failed to fetch Raydium price");
    
    assert!(raydium_price > 0);
    
    // Test Orca integration
    let orca_price = dex_client
        .fetch_price("orca", "SOL/USDC")
        .await
        .expect("Failed to fetch Orca price");
    
    assert!(orca_price > 0);
    
    // Prices should be close
    let diff = (raydium_price as i64 - orca_price as i64).abs();
    let avg = (raydium_price + orca_price) / 2;
    assert!((diff as f64 / avg as f64) < 0.05); // Less than 5% difference
}

#[tokio::test]
async fn test_multi_chain_balance_sync() {
    let sync_manager = MultiChainSyncManager::new_for_test().await;
    
    // Fetch balances from multiple chains
    let balances = sync_manager.sync_all_balances().await.unwrap();
    
    assert!(balances.len() > 0);
    assert!(balances.contains_key("solana"));
    
    // Verify total balance calculation
    let total = sync_manager.calculate_total_balance_usd(&balances);
    assert!(total > 0.0);
}
```

### 3. End-to-End Tests

Test complete workflows.

**Example: Full Arbitrage Cycle**

```rust
#[tokio::test]
async fn test_full_arbitrage_cycle() {
    // Setup test environment
    let mut test_env = TestEnvironment::new().await;
    let wallet = test_env.create_funded_wallet(10_000_000_000).await; // 10 SOL
    
    // Initialize arbitrage state
    let state = test_env.initialize_arbitrage_state(&wallet).await.unwrap();
    
    // Set up price difference between DEXs
    test_env.set_dex_price("raydium", "SOL/USDC", 100_000_000).await;
    test_env.set_dex_price("orca", "SOL/USDC", 102_000_000).await;
    
    // Execute arbitrage
    let result = test_env.execute_arbitrage(&state, 1_000_000_000).await;
    
    assert!(result.is_ok());
    
    let profit = result.unwrap();
    assert!(profit > 0);
    assert!(profit > state.min_profit);
    
    // Verify final balances
    let final_balance = test_env.get_wallet_balance(&wallet).await;
    assert!(final_balance > 10_000_000_000); // Should have profit
    
    // Cleanup
    test_env.cleanup().await;
}

#[tokio::test]
async fn test_governance_proposal_lifecycle() {
    let test_env = TestEnvironment::new().await;
    
    // Create proposal
    let proposal = test_env
        .create_proposal("Increase min_profit threshold")
        .await
        .unwrap();
    
    // Cast votes
    test_env.vote(&proposal, true, 1_000_000).await.unwrap();
    
    // Wait for voting period
    test_env.advance_time(Duration::from_secs(86400)).await; // 24 hours
    
    // Execute proposal
    let result = test_env.execute_proposal(&proposal).await;
    assert!(result.is_ok());
    
    // Verify state change
    let new_state = test_env.get_arbitrage_state().await;
    assert_eq!(new_state.min_profit, 200_000_000); // New threshold
}
```

### 4. Performance Tests

Benchmark critical operations.

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_arbitrage_calculation(c: &mut Criterion) {
    c.bench_function("calculate_arbitrage_profit", |b| {
        b.iter(|| {
            calculate_profit(
                black_box(100_000_000),
                black_box(101_000_000),
                black_box(1_000_000_000),
            )
        })
    });
}

fn benchmark_price_fetch(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let client = rt.block_on(async { DexClient::new(TestConfig::default()) });
    
    c.bench_function("fetch_dex_prices", |b| {
        b.to_async(&rt).iter(|| async {
            client.fetch_all_prices().await
        })
    });
}

criterion_group!(benches, benchmark_arbitrage_calculation, benchmark_price_fetch);
criterion_main!(benches);
```

### 5. Security Tests

Validate security measures.

```rust
#[tokio::test]
async fn test_unauthorized_access() {
    let test_env = TestEnvironment::new().await;
    let unauthorized_wallet = test_env.create_wallet().await;
    
    // Try to execute arbitrage without authorization
    let result = test_env
        .execute_arbitrage_as(&unauthorized_wallet, 1_000_000_000)
        .await;
    
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), Error::Unauthorized));
}

#[tokio::test]
async fn test_reentrancy_protection() {
    let test_env = TestEnvironment::new().await;
    
    // Attempt reentrancy attack
    let result = test_env.attempt_reentrancy_attack().await;
    
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), Error::ReentrancyDetected));
}

#[test]
fn test_input_validation() {
    // Test overflow protection
    let max_amount = u64::MAX;
    let result = validate_trade_amount(max_amount);
    assert!(result.is_err());
    
    // Test zero amount
    let result = validate_trade_amount(0);
    assert!(result.is_err());
    
    // Test negative values (should not compile, but test boundary)
    let valid_amount = 1_000_000_000;
    let result = validate_trade_amount(valid_amount);
    assert!(result.is_ok());
}
```

## 📊 Test Coverage

### Measuring Coverage

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html --output-dir coverage/

# View coverage
open coverage/index.html
```

### Coverage Requirements

```toml
[coverage]
minimum_coverage = 80
enforce_on_ci = true
exclude_files = [
    "tests/*",
    "benches/*",
    "examples/*"
]
```

## 🔄 Continuous Testing

### CI/CD Integration

**GitHub Actions Configuration**:

```yaml
name: Continuous Testing

on:
  push:
  pull_request:
  schedule:
    - cron: '0 */6 * * *'

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          
      - name: Run Tests
        run: |
          cargo test --all-features
          cargo test --release
          
      - name: Run Integration Tests
        run: cargo test --test integration_tests
        
      - name: Run Benchmarks
        run: cargo bench
        
      - name: Generate Coverage
        run: |
          cargo install cargo-tarpaulin
          cargo tarpaulin --out Xml
          
      - name: Upload Coverage
        uses: codecov/codecov-action@v1
```

## 📈 Test Reporting

### Test Results Dashboard

Access test results at: `http://localhost:9090/tests/dashboard`

**Report Contents:**
- Test pass/fail summary
- Coverage metrics
- Performance benchmarks
- Historical trends
- Flaky test detection

### Automated Notifications

```toml
[notifications]
enabled = true
on_failure = true
on_coverage_drop = true

[notifications.slack]
webhook_url = "https://hooks.slack.com/services/..."
channel = "#smsdao-tests"

[notifications.email]
recipients = ["dev@smsdao.io"]
```

## 🐛 Debugging Failed Tests

### View Test Output

```bash
# Run with detailed output
cargo test -- --nocapture --test-threads=1

# Run specific test with debug logging
RUST_LOG=debug cargo test test_name -- --nocapture
```

### Test Fixtures

```rust
// Setup common test fixtures
pub struct TestFixture {
    pub env: TestEnvironment,
    pub wallet: Keypair,
    pub state: Pubkey,
}

impl TestFixture {
    pub async fn new() -> Self {
        let env = TestEnvironment::new().await;
        let wallet = env.create_funded_wallet(10_000_000_000).await;
        let state = env.initialize_arbitrage_state(&wallet).await.unwrap();
        
        Self { env, wallet, state }
    }
}

#[tokio::test]
async fn test_with_fixture() {
    let fixture = TestFixture::new().await;
    // Use fixture.env, fixture.wallet, fixture.state
}
```

## 🎯 Best Practices

1. **Write Tests First**: Follow TDD principles
2. **Isolate Tests**: Each test should be independent
3. **Use Meaningful Names**: Test names should describe what they test
4. **Test Edge Cases**: Include boundary and error conditions
5. **Keep Tests Fast**: Optimize for quick feedback
6. **Mock External Services**: Use mocks for DEX/Oracle APIs
7. **Clean Up Resources**: Always cleanup test resources
8. **Document Test Intent**: Add comments explaining complex tests

## 🔗 Integration with Auto Fix

Auto Test integrates with [Auto Fix](AUTO_FIX.md) to automatically:
- Detect test failures
- Analyze root causes
- Suggest fixes
- Apply safe corrections

---

**Related Documentation:**
- [Auto Sync](AUTO_SYNC.md)
- [Auto Analysis](AUTO_ANALYSIS.md)
- [Auto Fix](AUTO_FIX.md)
- [Contributing Guide](CONTRIBUTING.md)
