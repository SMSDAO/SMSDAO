# API Reference

Complete API reference for SMSDAO smart contracts and services.

## 📋 Overview

This document provides comprehensive API documentation for:
- Smart contract interfaces
- Program instructions
- Account structures
- Events and errors
- RPC endpoints

## 🔗 Smart Contract Interfaces

### Arbitrage Bot Program

**Program ID**: `YourProgramIDHere` (update after deployment)

#### Instructions

##### `initialize`

Initializes the arbitrage state account.

**Parameters:**
```rust
pub fn initialize(
    ctx: Context<Initialize>,
    min_profit: u64,
    dex1_program: Pubkey,
    dex2_program: Pubkey,
) -> Result<()>
```

**Arguments:**
- `ctx`: Account context containing required accounts
- `min_profit`: Minimum profit threshold in lamports
- `dex1_program`: First DEX program ID (e.g., Raydium)
- `dex2_program`: Second DEX program ID (e.g., Orca)

**Accounts:**
```rust
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 32 + 32 + 32 + 8)]
    pub state: Account<'info, ArbitrageState>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub token_a_vault: Account<'info, TokenAccount>,
    pub token_b_vault: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}
```

**Example (TypeScript):**
```typescript
const tx = await program.methods
  .initialize(
    new anchor.BN(100_000_000), // min_profit
    raydiumProgramId,
    orcaProgramId
  )
  .accounts({
    state: stateAccount.publicKey,
    owner: wallet.publicKey,
    tokenAVault: tokenAVaultPubkey,
    tokenBVault: tokenBVaultPubkey,
    systemProgram: SystemProgram.programId,
  })
  .rpc();
```

**Returns:**
- Success: Transaction signature
- Error: `ArbitrageError` enum

##### `execute_arbitrage`

Executes an arbitrage trade between two DEXs.

**Parameters:**
```rust
pub fn execute_arbitrage(
    ctx: Context<ExecuteArbitrage>,
    amount: u64
) -> Result<()>
```

**Arguments:**
- `ctx`: Account context containing required accounts
- `amount`: Trade amount in token base units

**Accounts:**
```rust
#[derive(Accounts)]
pub struct ExecuteArbitrage<'info> {
    #[account(mut)]
    pub state: Account<'info, ArbitrageState>,
    #[account(mut)]
    pub token_a_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub token_b_vault: Account<'info, TokenAccount>,
    pub dex1_program: AccountInfo<'info>,
    pub dex2_program: AccountInfo<'info>,
    pub owner: Signer<'info>,
}
```

**Example (TypeScript):**
```typescript
const tx = await program.methods
  .executeArbitrage(new anchor.BN(1_000_000_000))
  .accounts({
    state: stateAccount.publicKey,
    tokenAVault: tokenAVaultPubkey,
    tokenBVault: tokenBVaultPubkey,
    dex1Program: raydiumProgramId,
    dex2Program: orcaProgramId,
    owner: wallet.publicKey,
  })
  .rpc();
```

**Returns:**
- Success: Transaction signature and profit amount
- Error: `ArbitrageError::NotProfitable` if trade is not profitable

**Example (Rust CLI):**
```rust
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::Keypair;

let client = RpcClient::new("https://api.devnet.solana.com");
let payer = Keypair::new();

// Build and send transaction
let tx = program
    .request()
    .accounts(accounts)
    .args(instruction_data)
    .signer(&payer)
    .send()?;
```

### Account Structures

#### `ArbitrageState`

Main state account for the arbitrage bot.

```rust
#[account]
pub struct ArbitrageState {
    pub owner: Pubkey,           // 32 bytes
    pub token_a_vault: Pubkey,   // 32 bytes
    pub token_b_vault: Pubkey,   // 32 bytes
    pub dex1_program: Pubkey,    // 32 bytes
    pub dex2_program: Pubkey,    // 32 bytes
    pub min_profit: u64,         // 8 bytes
}
```

**Total Size**: 168 bytes + 8 bytes (discriminator) = 176 bytes

**Field Descriptions:**
- `owner`: Authority that can update the state
- `token_a_vault`: Vault account for token A (e.g., SOL)
- `token_b_vault`: Vault account for token B (e.g., USDC)
- `dex1_program`: First DEX program to monitor
- `dex2_program`: Second DEX program to monitor
- `min_profit`: Minimum profit to execute trade (in lamports)

**Reading State (TypeScript):**
```typescript
const state = await program.account.arbitrageState.fetch(
  stateAccount.publicKey
);
console.log("Owner:", state.owner.toString());
console.log("Min Profit:", state.minProfit.toString());
```

## 🎯 Events

### `ArbitrageExecuted`

Emitted when an arbitrage trade is successfully executed.

```rust
#[event]
pub struct ArbitrageExecuted {
    pub profit: u64,      // Profit in lamports
    pub timestamp: i64,   // Unix timestamp
}
```

**Example Listener (TypeScript):**
```typescript
const listener = program.addEventListener(
  "ArbitrageExecuted",
  (event, slot) => {
    console.log("Arbitrage executed!");
    console.log("Profit:", event.profit.toString());
    console.log("Timestamp:", new Date(event.timestamp * 1000));
    console.log("Slot:", slot);
  }
);

// Later: remove listener
program.removeEventListener(listener);
```

## ⚠️ Error Codes

### `ArbitrageError`

```rust
#[error_code]
pub enum ArbitrageError {
    #[msg("Arbitrage is not profitable")]
    NotProfitable,      // Error code: 6000
}
```

**Error Handling (TypeScript):**
```typescript
try {
  await program.methods.executeArbitrage(amount).rpc();
} catch (error) {
  if (error.code === 6000) {
    console.log("Trade not profitable, skipping");
  } else {
    console.error("Unexpected error:", error);
  }
}
```

## 🌐 RPC Endpoints

### Health Check

**Endpoint**: `GET /health`

**Response:**
```json
{
  "status": "healthy",
  "version": "1.0.0",
  "uptime": 3600,
  "network": "devnet"
}
```

### System Metrics

**Endpoint**: `GET /metrics`

**Response:**
```json
{
  "transactions": {
    "total": 1000,
    "successful": 980,
    "failed": 20
  },
  "arbitrage": {
    "total_profit": "10000000000",
    "trade_count": 150,
    "success_rate": 0.98
  },
  "system": {
    "cpu_usage": 0.45,
    "memory_usage": 0.67,
    "active_connections": 5
  }
}
```

### Get Arbitrage Opportunities

**Endpoint**: `GET /arbitrage/opportunities`

**Query Parameters:**
- `min_profit` (optional): Minimum profit threshold
- `token_pair` (optional): Specific token pair

**Response:**
```json
{
  "opportunities": [
    {
      "token_a": "SOL",
      "token_b": "USDC",
      "dex1": "Raydium",
      "dex2": "Orca",
      "dex1_price": 100.5,
      "dex2_price": 101.2,
      "potential_profit": "7000000",
      "estimated_gas": "5000"
    }
  ],
  "timestamp": 1707339515
}
```

### Get Treasury Balance

**Endpoint**: `GET /treasury/balance`

**Response:**
```json
{
  "balances": [
    {
      "chain": "solana",
      "token": "SOL",
      "amount": "1000.5",
      "usd_value": 100500.0
    },
    {
      "chain": "solana",
      "token": "USDC",
      "amount": "50000.0",
      "usd_value": 50000.0
    }
  ],
  "total_usd": 150500.0
}
```

### Get Governance Proposals

**Endpoint**: `GET /governance/proposals`

**Query Parameters:**
- `status` (optional): Filter by status (active, passed, rejected)
- `page` (optional): Page number (default: 1)
- `limit` (optional): Items per page (default: 10)

**Response:**
```json
{
  "proposals": [
    {
      "id": 1,
      "title": "Increase minimum profit threshold",
      "description": "Proposal to increase min_profit to 200000000",
      "proposer": "7yKZw...Abc",
      "status": "active",
      "votes_for": 1000000,
      "votes_against": 50000,
      "created_at": 1707339515,
      "voting_ends_at": 1707425915
    }
  ],
  "pagination": {
    "page": 1,
    "limit": 10,
    "total": 5
  }
}
```

## 🔧 Integration Examples

### Full Integration Example (TypeScript)

```typescript
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";

// Initialize connection
const connection = new anchor.web3.Connection(
  "https://api.devnet.solana.com"
);
const wallet = anchor.Wallet.local();
const provider = new anchor.AnchorProvider(connection, wallet, {});
anchor.setProvider(provider);

// Load program
const programId = new PublicKey("YourProgramIDHere");
const idl = await Program.fetchIdl(programId, provider);
const program = new Program(idl, programId, provider);

// Initialize arbitrage state
const stateKeypair = anchor.web3.Keypair.generate();
await program.methods
  .initialize(
    new anchor.BN(100_000_000),
    new PublicKey("RaydiumProgramID"),
    new PublicKey("OrcaProgramID")
  )
  .accounts({
    state: stateKeypair.publicKey,
    owner: wallet.publicKey,
    tokenAVault: tokenAVault,
    tokenBVault: tokenBVault,
    systemProgram: anchor.web3.SystemProgram.programId,
  })
  .signers([stateKeypair])
  .rpc();

// Execute arbitrage
await program.methods
  .executeArbitrage(new anchor.BN(1_000_000_000))
  .accounts({
    state: stateKeypair.publicKey,
    tokenAVault: tokenAVault,
    tokenBVault: tokenBVault,
    dex1Program: raydiumProgramId,
    dex2Program: orcaProgramId,
    owner: wallet.publicKey,
  })
  .rpc();
```

### Python Integration Example

```python
from solana.rpc.api import Client
from solana.keypair import Keypair
from solana.publickey import PublicKey
from anchorpy import Program, Provider, Wallet

# Initialize
client = Client("https://api.devnet.solana.com")
wallet = Wallet.local()
provider = Provider(client, wallet)

# Load program
program_id = PublicKey("YourProgramIDHere")
program = await Program.at(program_id, provider)

# Execute arbitrage
tx = await program.rpc["execute_arbitrage"](
    1_000_000_000,
    ctx=Context(
        accounts={
            "state": state_pubkey,
            "token_a_vault": token_a_vault,
            "token_b_vault": token_b_vault,
            "dex1_program": raydium_program_id,
            "dex2_program": orca_program_id,
            "owner": wallet.public_key,
        }
    ),
)
```

## 📝 Best Practices

1. **Transaction Retry Logic**
   - Implement exponential backoff
   - Handle rate limiting
   - Monitor transaction status

2. **Error Handling**
   - Always catch and log errors
   - Implement fallback mechanisms
   - Monitor error rates

3. **Security**
   - Validate all inputs
   - Use secure RPC endpoints
   - Protect private keys

4. **Performance**
   - Batch operations when possible
   - Use connection pooling
   - Cache frequently accessed data

---

**Related Documentation:**
- [Architecture Overview](ARCHITECTURE.md)
- [Getting Started Guide](GETTING_STARTED.md)
- [Security Documentation](SECURITY.md)
