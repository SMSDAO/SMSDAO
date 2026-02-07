# Deployment Guide

Complete guide for deploying SMSDAO to production environments.

## 🎯 Overview

This guide covers deployment to:
- Solana Mainnet
- Multi-chain environments
- Production infrastructure
- Monitoring and maintenance

## 📋 Pre-Deployment Checklist

### 1. Code Preparation

- [ ] All tests passing (`cargo test`)
- [ ] Security audit completed
- [ ] Code review approved
- [ ] Documentation updated
- [ ] Version tagged in git

### 2. Configuration

- [ ] Production config files created
- [ ] Environment variables set
- [ ] Secrets securely stored
- [ ] RPC endpoints configured
- [ ] Monitoring configured

### 3. Infrastructure

- [ ] Servers provisioned
- [ ] Network security configured
- [ ] Backup systems ready
- [ ] CI/CD pipeline configured
- [ ] Domain/DNS configured

### 4. Security

- [ ] Keys generated securely
- [ ] Multi-sig wallets configured
- [ ] Access control configured
- [ ] Firewall rules set
- [ ] Audit logging enabled

## 🚀 Deployment Steps

### Step 1: Build for Production

```bash
# Clean build
cargo clean

# Build in release mode with optimizations
cargo build --release

# Verify binary
./target/release/smsdao --version

# Run final tests
cargo test --release

# Create distribution package
tar -czf smsdao-v1.0.0.tar.gz \
  target/release/smsdao \
  config/ \
  docs/ \
  README.md
```

### Step 2: Deploy Smart Contracts

#### Anchor Program Deployment

```bash
# Set Solana cluster to mainnet
solana config set --url mainnet-beta

# Check balance (need enough for deployment)
solana balance

# Build the program
anchor build

# Get program ID
anchor keys list

# Update program ID in code
# Edit: Anchor.toml, src/main.rs

# Rebuild with correct program ID
anchor build

# Deploy to mainnet
anchor deploy --provider.cluster mainnet

# Verify deployment
solana program show <PROGRAM_ID>

# Initialize program state
anchor run initialize --provider.cluster mainnet
```

#### Verify Smart Contract

```bash
# Verify program deployment
solana program dump <PROGRAM_ID> program_dump.so

# Compare with local build
diff program_dump.so target/deploy/smsdao.so

# Test basic functionality
anchor test --skip-local-validator
```

### Step 3: Server Deployment

#### Using Docker

**Dockerfile:**
```dockerfile
FROM rust:1.70 as builder

WORKDIR /app
COPY . .

RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/smsdao /usr/local/bin/smsdao
COPY --from=builder /app/config /etc/smsdao/config

ENV CONFIG_PATH=/etc/smsdao/config/production.toml
ENV LOG_PATH=/var/log/smsdao

EXPOSE 9090

ENTRYPOINT ["smsdao"]
CMD ["--config", "/etc/smsdao/config/production.toml"]
```

**docker-compose.yml:**
```yaml
version: '3.8'

services:
  smsdao:
    build: .
    image: smsdao:latest
    container_name: smsdao-prod
    restart: unless-stopped
    ports:
      - "9090:9090"
    environment:
      - SOLANA_NETWORK=mainnet-beta
      - SOLANA_RPC_URL=${SOLANA_RPC_URL}
      - PROGRAM_ID=${PROGRAM_ID}
    env_file:
      - .env.production
    volumes:
      - ./logs:/var/log/smsdao
      - ./config:/etc/smsdao/config:ro
      - ./keys:/etc/smsdao/keys:ro
    networks:
      - smsdao-network
    logging:
      driver: "json-file"
      options:
        max-size: "100m"
        max-file: "10"

  prometheus:
    image: prom/prometheus:latest
    container_name: prometheus
    restart: unless-stopped
    ports:
      - "9091:9090"
    volumes:
      - ./prometheus.yml:/etc/prometheus/prometheus.yml:ro
      - prometheus-data:/prometheus
    networks:
      - smsdao-network

  grafana:
    image: grafana/grafana:latest
    container_name: grafana
    restart: unless-stopped
    ports:
      - "3000:3000"
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=${GRAFANA_PASSWORD}
    volumes:
      - grafana-data:/var/lib/grafana
    networks:
      - smsdao-network

volumes:
  prometheus-data:
  grafana-data:

networks:
  smsdao-network:
    driver: bridge
```

**Deploy with Docker:**
```bash
# Build image
docker build -t smsdao:v1.0.0 .

# Tag for registry
docker tag smsdao:v1.0.0 registry.example.com/smsdao:v1.0.0

# Push to registry
docker push registry.example.com/smsdao:v1.0.0

# Deploy
docker-compose up -d

# Check logs
docker-compose logs -f smsdao

# Check health
curl http://localhost:9090/health
```

#### Using Systemd

**systemd service file** (`/etc/systemd/system/smsdao.service`):
```ini
[Unit]
Description=SMSDAO Arbitrage Bot
After=network.target

[Service]
Type=simple
User=smsdao
Group=smsdao
WorkingDirectory=/opt/smsdao
EnvironmentFile=/opt/smsdao/.env.production
ExecStart=/opt/smsdao/smsdao --config /opt/smsdao/config/production.toml
Restart=always
RestartSec=10
StandardOutput=append:/var/log/smsdao/smsdao.log
StandardError=append:/var/log/smsdao/error.log

# Security hardening
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/var/log/smsdao

[Install]
WantedBy=multi-user.target
```

**Deploy with systemd:**
```bash
# Create user
sudo useradd -r -s /bin/false smsdao

# Create directories
sudo mkdir -p /opt/smsdao/{config,keys,logs}
sudo mkdir -p /var/log/smsdao

# Copy files
sudo cp target/release/smsdao /opt/smsdao/
sudo cp -r config/* /opt/smsdao/config/
sudo cp .env.production /opt/smsdao/

# Set permissions
sudo chown -R smsdao:smsdao /opt/smsdao
sudo chown -R smsdao:smsdao /var/log/smsdao
sudo chmod 600 /opt/smsdao/.env.production
sudo chmod 600 /opt/smsdao/keys/*

# Install service
sudo cp smsdao.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable smsdao
sudo systemctl start smsdao

# Check status
sudo systemctl status smsdao

# View logs
sudo journalctl -u smsdao -f
```

### Step 4: Network Configuration

#### RPC Endpoint Setup

**Primary RPC:**
- Use dedicated RPC nodes
- Configure rate limits
- Set up fallbacks

**Recommended Providers:**
- Solana mainnet: Helius, Triton, QuickNode
- Base: Alchemy, Infura
- Self-hosted nodes for critical operations

**Example configuration:**
```toml
[network]
solana_rpc_url = "https://mainnet.helius-rpc.com/?api-key=YOUR_KEY"

[network.fallback_rpcs]
endpoints = [
    "https://api.mainnet-beta.solana.com",
    "https://solana-api.projectserum.com",
    "https://rpc.ankr.com/solana",
]
```

#### Firewall Configuration

```bash
# Allow SSH (from specific IPs only)
sudo ufw allow from 192.168.1.0/24 to any port 22

# Allow metrics endpoint (internal network only)
sudo ufw allow from 10.0.0.0/8 to any port 9090

# Allow HTTPS (if exposing API)
sudo ufw allow 443

# Enable firewall
sudo ufw enable

# Check status
sudo ufw status verbose
```

#### Load Balancer Setup

**nginx configuration:**
```nginx
upstream smsdao_backend {
    least_conn;
    server 10.0.1.10:9090 max_fails=3 fail_timeout=30s;
    server 10.0.1.11:9090 max_fails=3 fail_timeout=30s;
    server 10.0.1.12:9090 max_fails=3 fail_timeout=30s;
}

server {
    listen 443 ssl http2;
    server_name api.smsdao.io;

    ssl_certificate /etc/ssl/certs/smsdao.crt;
    ssl_certificate_key /etc/ssl/private/smsdao.key;

    location / {
        proxy_pass http://smsdao_backend;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;

        # Timeouts
        proxy_connect_timeout 30s;
        proxy_send_timeout 30s;
        proxy_read_timeout 30s;

        # Rate limiting
        limit_req zone=api_limit burst=20 nodelay;
    }

    location /metrics {
        # Restrict to internal network
        allow 10.0.0.0/8;
        deny all;
        
        proxy_pass http://smsdao_backend;
    }
}

# Rate limiting zone
limit_req_zone $binary_remote_addr zone=api_limit:10m rate=10r/s;
```

### Step 5: Monitoring Setup

#### Prometheus Configuration

**prometheus.yml:**
```yaml
global:
  scrape_interval: 15s
  evaluation_interval: 15s
  external_labels:
    cluster: 'smsdao-production'

alerting:
  alertmanagers:
    - static_configs:
        - targets: ['localhost:9093']

scrape_configs:
  - job_name: 'smsdao'
    static_configs:
      - targets: ['smsdao:9090']
    metrics_path: '/metrics'

  - job_name: 'node'
    static_configs:
      - targets: ['node-exporter:9100']
```

#### Grafana Dashboards

Import pre-built dashboards for:
- System metrics
- Transaction monitoring
- Arbitrage performance
- Error tracking

#### Alert Rules

**alerts.yml:**
```yaml
groups:
  - name: smsdao_alerts
    interval: 30s
    rules:
      - alert: HighErrorRate
        expr: rate(smsdao_errors_total[5m]) > 0.05
        for: 5m
        labels:
          severity: critical
        annotations:
          summary: "High error rate detected"
          description: "Error rate is {{ $value }} errors/second"

      - alert: LowSuccessRate
        expr: rate(smsdao_successful_trades[5m]) / rate(smsdao_total_trades[5m]) < 0.95
        for: 10m
        labels:
          severity: warning
        annotations:
          summary: "Low trade success rate"

      - alert: ServiceDown
        expr: up{job="smsdao"} == 0
        for: 1m
        labels:
          severity: critical
        annotations:
          summary: "SMSDAO service is down"
```

### Step 6: Database Setup

```bash
# PostgreSQL setup
sudo apt-get install postgresql-14

# Create database and user
sudo -u postgres psql
CREATE DATABASE smsdao;
CREATE USER smsdao_user WITH ENCRYPTED PASSWORD 'secure_password';
GRANT ALL PRIVILEGES ON DATABASE smsdao TO smsdao_user;

# Run migrations
cargo run --bin migrate

# Backup setup
sudo crontab -e
# Add: 0 2 * * * pg_dump smsdao > /backups/smsdao_$(date +\%Y\%m\%d).sql
```

## 🔒 Security Hardening

### 1. Key Management

```bash
# Generate production keys securely (offline)
solana-keygen new --outfile production-wallet.json --no-bip39-passphrase

# Store in secure location
# - Hardware wallet for program authority
# - HSM for hot wallet
# - Multi-sig for treasury

# Restrict permissions
chmod 400 production-wallet.json
```

### 2. Multi-Signature Setup

```bash
# Create multi-sig account
solana-keygen new --outfile member1.json
solana-keygen new --outfile member2.json
solana-keygen new --outfile member3.json

# Create 2-of-3 multi-sig (using Squads or similar)
# This requires web interface or custom tooling
```

### 3. Access Control

- Use firewall rules
- VPN for administrative access
- 2FA for all accounts
- Regular security audits
- Principle of least privilege

## 📊 Post-Deployment Verification

### Health Checks

```bash
# System health
curl http://localhost:9090/health

# Metrics
curl http://localhost:9090/metrics

# Test trade execution (with small amount)
curl -X POST http://localhost:9090/arbitrage/test \
  -H "Content-Type: application/json" \
  -d '{"amount": 1000000}'

# Check logs
tail -f /var/log/smsdao/smsdao.log
```

### Smoke Tests

```bash
# Run post-deployment tests
./scripts/smoke-tests.sh production

# Verify all integrations
./scripts/verify-integrations.sh

# Test failover
./scripts/test-failover.sh
```

## 🔄 Continuous Deployment

### CI/CD Pipeline

**GitHub Actions** (`.github/workflows/deploy.yml`):
```yaml
name: Deploy to Production

on:
  push:
    tags:
      - 'v*'

jobs:
  deploy:
    runs-on: ubuntu-latest
    environment: production
    steps:
      - uses: actions/checkout@v2
      
      - name: Build
        run: cargo build --release
      
      - name: Run tests
        run: cargo test --release
      
      - name: Build Docker image
        run: docker build -t smsdao:${{ github.ref_name }} .
      
      - name: Push to registry
        run: |
          docker login -u ${{ secrets.DOCKER_USERNAME }} -p ${{ secrets.DOCKER_PASSWORD }}
          docker push smsdao:${{ github.ref_name }}
      
      - name: Deploy to production
        run: |
          ssh ${{ secrets.PROD_SERVER }} "cd /opt/smsdao && docker-compose pull && docker-compose up -d"
      
      - name: Verify deployment
        run: |
          ./scripts/verify-deployment.sh
```

## 🔙 Rollback Procedures

### Quick Rollback

```bash
# Stop current version
sudo systemctl stop smsdao

# Restore previous version
sudo cp /opt/smsdao/backups/smsdao.v0.9.0 /opt/smsdao/smsdao

# Restart
sudo systemctl start smsdao

# Verify
curl http://localhost:9090/health
```

### Database Rollback

```bash
# Restore database from backup
pg_restore -d smsdao /backups/smsdao_20260207.sql

# Run reverse migrations if needed
cargo run --bin migrate -- --down
```

## 📚 Maintenance

### Regular Tasks

**Daily:**
- Check system health
- Review error logs
- Monitor metrics
- Verify backups

**Weekly:**
- Review performance metrics
- Update dependencies
- Security scans
- Capacity planning

**Monthly:**
- Full system audit
- Update documentation
- Review access logs
- Test disaster recovery

### Updates and Upgrades

```bash
# Minor updates
git pull origin main
cargo build --release
sudo systemctl restart smsdao

# Major upgrades
# Follow full deployment process
# Test on staging first
# Plan maintenance window
```

---

**Related Documentation:**
- [Configuration Guide](CONFIGURATION.md)
- [Security Documentation](SECURITY.md)
- [Troubleshooting](TROUBLESHOOTING.md)
