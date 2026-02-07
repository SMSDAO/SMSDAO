# Auto Analysis

AI-driven market analysis, performance monitoring, and intelligent insights for SMSDAO operations.

## 🎯 Overview

Auto Analysis provides intelligent automation for:
- Real-time market opportunity detection
- Performance analytics and optimization
- Anomaly detection
- Predictive modeling
- Risk assessment
- Trading pattern analysis

## 🧠 Analysis Components

### 1. Market Opportunity Analysis

**Purpose**: Automatically identify profitable arbitrage opportunities.

**Configuration** (`config/auto_analysis.toml`):
```toml
[market_analysis]
enabled = true
scan_interval_ms = 500
min_profit_threshold = 100000000  # 0.1 SOL

[market_analysis.algorithms]
price_spread_detection = true
volume_analysis = true
liquidity_assessment = true
trend_prediction = true

[market_analysis.ml_models]
enabled = true
model_path = "models/arbitrage_predictor.onnx"
confidence_threshold = 0.75
retrain_interval_hours = 24
```

**Features:**
- Multi-DEX price spread analysis
- Liquidity depth evaluation
- Slippage estimation
- Gas cost optimization
- Profitability scoring

**Implementation:**
```rust
pub struct MarketAnalyzer {
    dex_clients: Vec<DexClient>,
    price_history: Arc<Mutex<PriceHistory>>,
    ml_model: Option<PredictorModel>,
}

impl MarketAnalyzer {
    pub async fn analyze_opportunities(&self) -> Result<Vec<OpportunityScore>> {
        let mut opportunities = Vec::new();
        
        // Fetch current prices from all DEXs
        let prices = self.fetch_all_prices().await?;
        
        // Analyze each token pair
        for pair in self.get_monitored_pairs() {
            let analysis = self.analyze_pair(&pair, &prices).await?;
            
            if analysis.score > self.config.confidence_threshold {
                opportunities.push(OpportunityScore {
                    pair,
                    expected_profit: analysis.expected_profit,
                    confidence: analysis.score,
                    execution_plan: analysis.plan,
                    risk_level: analysis.risk,
                });
            }
        }
        
        // Sort by expected profit adjusted by confidence
        opportunities.sort_by(|a, b| {
            let a_score = a.expected_profit as f64 * a.confidence;
            let b_score = b.expected_profit as f64 * b.confidence;
            b_score.partial_cmp(&a_score).unwrap()
        });
        
        Ok(opportunities)
    }
    
    fn analyze_pair(&self, pair: &TradingPair, prices: &PriceMap) 
        -> Result<PairAnalysis> {
        let dex_prices = prices.get_pair_prices(pair);
        
        // Calculate spread
        let (min_price, max_price) = dex_prices.min_max();
        let spread = max_price - min_price;
        let spread_percent = (spread as f64 / min_price as f64) * 100.0;
        
        // Estimate slippage
        let slippage = self.estimate_slippage(pair, dex_prices)?;
        
        // Calculate gas costs
        let gas_cost = self.estimate_gas_cost(pair)?;
        
        // Calculate net profit
        let gross_profit = spread - gas_cost;
        let net_profit = gross_profit - slippage;
        
        // ML-based confidence scoring
        let confidence = if let Some(model) = &self.ml_model {
            model.predict_success_probability(&PairAnalysis {
                spread_percent,
                slippage,
                gas_cost,
                liquidity: dex_prices.avg_liquidity(),
            })?
        } else {
            self.heuristic_confidence(spread_percent, slippage)
        };
        
        Ok(PairAnalysis {
            expected_profit: net_profit,
            score: confidence,
            risk: self.assess_risk(pair, dex_prices),
            plan: self.create_execution_plan(pair, dex_prices),
        })
    }
}
```

### 2. Performance Analytics

**Purpose**: Monitor and optimize system performance.

**Metrics Tracked:**
- Transaction success rates
- Average execution time
- Profit per trade
- Gas efficiency
- Slippage accuracy
- Capital utilization

**Dashboard Metrics:**
```json
{
  "performance": {
    "today": {
      "trades_executed": 145,
      "success_rate": 0.98,
      "total_profit_sol": 12.5,
      "avg_profit_per_trade": 0.086,
      "avg_execution_time_ms": 450,
      "total_gas_spent_sol": 0.5
    },
    "trends": {
      "profit_trend": "increasing",
      "success_rate_trend": "stable",
      "execution_time_trend": "improving"
    }
  }
}
```

**Analytics Queries:**
```rust
pub async fn generate_performance_report(&self, period: TimePeriod) 
    -> Result<PerformanceReport> {
    let trades = self.db.fetch_trades(period).await?;
    
    let report = PerformanceReport {
        total_trades: trades.len(),
        successful_trades: trades.iter().filter(|t| t.success).count(),
        total_profit: trades.iter().map(|t| t.profit).sum(),
        avg_profit: trades.iter().map(|t| t.profit).sum::<u64>() 
                    / trades.len() as u64,
        best_trade: trades.iter().max_by_key(|t| t.profit),
        worst_trade: trades.iter().min_by_key(|t| t.profit),
        total_gas: trades.iter().map(|t| t.gas_used).sum(),
        avg_execution_time: trades.iter()
            .map(|t| t.execution_time)
            .sum::<Duration>() / trades.len() as u32,
    };
    
    Ok(report)
}
```

### 3. Anomaly Detection

**Purpose**: Detect unusual patterns that may indicate issues or opportunities.

**Detection Methods:**
- Statistical outlier detection
- Pattern deviation analysis
- Behavioral anomalies
- Network anomalies

**Configuration:**
```toml
[anomaly_detection]
enabled = true
sensitivity = "medium"  # low, medium, high
check_interval_ms = 5000

[anomaly_detection.rules]
price_spike_threshold = 0.2      # 20% sudden change
volume_spike_threshold = 3.0      # 3x average volume
slippage_threshold = 0.05         # 5% slippage
execution_time_threshold = 5000   # 5 seconds

[anomaly_detection.alerts]
enabled = true
severity_levels = ["warning", "critical"]
notification_channels = ["slack", "email"]
```

**Implementation:**
```rust
pub struct AnomalyDetector {
    baseline_metrics: Arc<Mutex<BaselineMetrics>>,
    alert_manager: AlertManager,
}

impl AnomalyDetector {
    pub async fn check_for_anomalies(&self) -> Result<Vec<Anomaly>> {
        let mut anomalies = Vec::new();
        
        // Check price anomalies
        if let Some(anomaly) = self.check_price_anomalies().await? {
            anomalies.push(anomaly);
        }
        
        // Check volume anomalies
        if let Some(anomaly) = self.check_volume_anomalies().await? {
            anomalies.push(anomaly);
        }
        
        // Check execution anomalies
        if let Some(anomaly) = self.check_execution_anomalies().await? {
            anomalies.push(anomaly);
        }
        
        // Send alerts for critical anomalies
        for anomaly in &anomalies {
            if anomaly.severity == Severity::Critical {
                self.alert_manager.send_alert(anomaly).await?;
            }
        }
        
        Ok(anomalies)
    }
    
    async fn check_price_anomalies(&self) -> Result<Option<Anomaly>> {
        let current_prices = self.fetch_current_prices().await?;
        let baseline = self.baseline_metrics.lock().await;
        
        for (pair, price) in current_prices {
            if let Some(avg_price) = baseline.avg_price.get(&pair) {
                let deviation = ((price as f64 - *avg_price) / *avg_price).abs();
                
                if deviation > self.config.price_spike_threshold {
                    return Ok(Some(Anomaly {
                        anomaly_type: AnomalyType::PriceSpike,
                        severity: Severity::Warning,
                        description: format!(
                            "Price spike detected for {}: {:.2}% deviation",
                            pair, deviation * 100.0
                        ),
                        timestamp: Utc::now(),
                        data: serde_json::json!({
                            "pair": pair,
                            "current_price": price,
                            "baseline_price": avg_price,
                            "deviation": deviation,
                        }),
                    }));
                }
            }
        }
        
        Ok(None)
    }
}
```

### 4. Predictive Modeling

**Purpose**: Use machine learning to predict market movements and optimize strategies.

**Models:**
- Price trend prediction
- Volatility forecasting
- Liquidity prediction
- Success probability estimation

**Model Training:**
```rust
pub struct ModelTrainer {
    training_data: TrainingDataset,
    model_config: ModelConfig,
}

impl ModelTrainer {
    pub async fn train_arbitrage_model(&mut self) -> Result<PredictorModel> {
        // Collect historical trade data
        let historical_trades = self.fetch_historical_data().await?;
        
        // Feature engineering
        let features = self.extract_features(&historical_trades)?;
        let labels = self.extract_labels(&historical_trades)?;
        
        // Train model
        let model = self.train_model(features, labels)?;
        
        // Validate model
        let validation_score = self.validate_model(&model).await?;
        
        if validation_score > self.config.min_validation_score {
            // Save model
            self.save_model(&model, "models/arbitrage_predictor.onnx")?;
            Ok(model)
        } else {
            Err(Error::ModelValidationFailed(validation_score))
        }
    }
    
    fn extract_features(&self, trades: &[Trade]) -> Result<Vec<Features>> {
        trades.iter().map(|trade| {
            Ok(Features {
                spread_percent: trade.spread_percent(),
                liquidity_ratio: trade.liquidity_ratio(),
                volatility: trade.volatility(),
                time_of_day: trade.timestamp.hour(),
                day_of_week: trade.timestamp.weekday() as u8,
                recent_volume: trade.recent_volume(),
                gas_price: trade.gas_price,
            })
        }).collect()
    }
}
```

### 5. Risk Assessment

**Purpose**: Continuously assess and quantify operational risks.

**Risk Categories:**
- Market risk (volatility, liquidity)
- Execution risk (slippage, gas)
- Smart contract risk (exploits, bugs)
- Operational risk (downtime, errors)

**Risk Scoring:**
```rust
pub struct RiskAssessor {
    risk_models: Vec<RiskModel>,
}

impl RiskAssessor {
    pub async fn assess_trade_risk(&self, trade: &ProposedTrade) 
        -> Result<RiskScore> {
        let mut risk_factors = Vec::new();
        
        // Market risk
        let market_risk = self.assess_market_risk(trade).await?;
        risk_factors.push(market_risk);
        
        // Execution risk
        let execution_risk = self.assess_execution_risk(trade).await?;
        risk_factors.push(execution_risk);
        
        // Liquidity risk
        let liquidity_risk = self.assess_liquidity_risk(trade).await?;
        risk_factors.push(liquidity_risk);
        
        // Calculate composite risk score
        let composite_score = self.calculate_composite_risk(&risk_factors);
        
        Ok(RiskScore {
            overall: composite_score,
            market: market_risk,
            execution: execution_risk,
            liquidity: liquidity_risk,
            recommendation: self.get_recommendation(composite_score),
        })
    }
    
    async fn assess_market_risk(&self, trade: &ProposedTrade) 
        -> Result<f64> {
        // Analyze volatility
        let volatility = self.calculate_volatility(&trade.pair).await?;
        
        // Analyze correlation with broader market
        let correlation = self.calculate_correlation(&trade.pair).await?;
        
        // Risk score (0-1, where 1 is highest risk)
        let risk = (volatility * 0.6) + (correlation.abs() * 0.4);
        
        Ok(risk.clamp(0.0, 1.0))
    }
}
```

## 🚀 Usage

### Enable Auto Analysis

```bash
# Enable all analysis features
cargo run --release -- --enable-auto-analysis

# Enable specific analysis components
cargo run --release -- \
  --enable-market-analysis \
  --enable-performance-analytics \
  --enable-anomaly-detection
```

### Programmatic Usage

**Rust:**
```rust
use smsdao::analysis::{AutoAnalysisManager, AnalysisConfig};

#[tokio::main]
async fn main() -> Result<()> {
    let config = AnalysisConfig::from_file("config/auto_analysis.toml")?;
    let analyzer = AutoAnalysisManager::new(config);
    
    // Start analysis tasks
    analyzer.start_all().await?;
    
    // Query analysis results
    loop {
        let opportunities = analyzer.get_top_opportunities().await?;
        println!("Top opportunities: {:?}", opportunities);
        
        let performance = analyzer.get_performance_metrics().await?;
        println!("Performance: {:?}", performance);
        
        let anomalies = analyzer.get_recent_anomalies().await?;
        if !anomalies.is_empty() {
            println!("Anomalies detected: {:?}", anomalies);
        }
        
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}
```

**TypeScript:**
```typescript
import { AutoAnalysisManager } from '@smsdao/sdk';

const analyzer = new AutoAnalysisManager({
  marketAnalysis: { enabled: true, scanIntervalMs: 500 },
  performanceAnalytics: { enabled: true },
  anomalyDetection: { enabled: true, sensitivity: 'medium' },
});

await analyzer.start();

// Subscribe to analysis events
analyzer.on('opportunityDetected', (opportunity) => {
  console.log('New opportunity:', opportunity);
  // Evaluate and execute if profitable
});

analyzer.on('anomalyDetected', (anomaly) => {
  console.warn('Anomaly detected:', anomaly);
  // Take appropriate action
});

analyzer.on('performanceReport', (report) => {
  console.log('Performance report:', report);
});
```

## 📊 Analysis Dashboard

Access the analysis dashboard at: `http://localhost:9090/analysis/dashboard`

**Dashboard Sections:**
1. Market Opportunities
2. Performance Metrics
3. Anomaly Alerts
4. Risk Assessment
5. Predictive Insights

## 📈 Reporting

### Generate Analysis Reports

```bash
# Daily performance report
curl http://localhost:9090/analysis/report/daily

# Weekly analysis summary
curl http://localhost:9090/analysis/report/weekly

# Custom period report
curl "http://localhost:9090/analysis/report?from=2026-02-01&to=2026-02-07"
```

**Report Format:**
```json
{
  "period": "2026-02-01 to 2026-02-07",
  "summary": {
    "total_opportunities": 500,
    "executed_trades": 145,
    "total_profit_sol": 12.5,
    "success_rate": 0.98,
    "avg_execution_time_ms": 450
  },
  "top_opportunities": [...],
  "anomalies": [...],
  "risk_events": [...],
  "recommendations": [...]
}
```

## 🔧 Advanced Configuration

### Custom Analysis Rules

```toml
[custom_rules]
# Define custom opportunity scoring
[[custom_rules.opportunity_scoring]]
name = "high_volume_filter"
condition = "volume > 1000000 AND spread > 0.01"
score_multiplier = 1.5

[[custom_rules.opportunity_scoring]]
name = "low_risk_boost"
condition = "risk_level < 0.3"
score_multiplier = 1.2

# Define custom anomaly detection
[[custom_rules.anomaly_detection]]
name = "flash_crash_detector"
condition = "price_change < -0.15 AND time_window < 60"
severity = "critical"
```

## 🧠 Machine Learning Integration

### Model Management

```bash
# Train new model
cargo run --bin train_model -- --data historical_trades.csv

# Evaluate model
cargo run --bin evaluate_model -- --model models/arbitrage_predictor.onnx

# Deploy model
cp models/arbitrage_predictor.onnx production/models/
```

### Model Monitoring

Monitor model performance:
- Prediction accuracy
- False positive rate
- Response time
- Drift detection

## 🔐 Security Considerations

1. **Data Privacy**: Anonymize sensitive trading data
2. **Model Security**: Protect ML models from adversarial attacks
3. **API Security**: Secure analysis endpoints
4. **Audit Trail**: Log all analysis decisions

## 📚 Best Practices

1. **Regular Calibration**: Update baseline metrics regularly
2. **Model Retraining**: Retrain ML models with new data
3. **Alert Tuning**: Adjust anomaly thresholds based on false positive rates
4. **Performance Monitoring**: Track analysis system performance
5. **Backtesting**: Validate analysis results against historical data

---

**Related Documentation:**
- [Auto Sync](AUTO_SYNC.md)
- [Auto Test](AUTO_TEST.md)
- [Auto Fix](AUTO_FIX.md)
- [Architecture](ARCHITECTURE.md)
