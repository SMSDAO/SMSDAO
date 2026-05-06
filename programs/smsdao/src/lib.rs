use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

// Define the program ID
declare_id!("779kqQC5yYMBR5KVjbExg3sSnQUU1Z1p8fPYApirXML3");

// Program state to store arbitrage settings
#[account]
pub struct ArbitrageState {
    pub owner: Pubkey,
    pub token_a_vault: Pubkey, // Vault for token A (e.g., SOL)
    pub token_b_vault: Pubkey, // Vault for token B (e.g., USDC)
    pub dex1_program: Pubkey,  // DEX1 program ID (e.g., Raydium)
    pub dex2_program: Pubkey,  // DEX2 program ID (e.g., Orca)
    pub min_profit: u64,       // Minimum profit threshold (in lamports)
}

#[program]
pub mod arbitrage_bot {
    use super::*;

    // Initialize the arbitrage state
    pub fn initialize(
        ctx: Context<Initialize>,
        min_profit: u64,
        dex1_program: Pubkey,
        dex2_program: Pubkey,
    ) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.owner = ctx.accounts.owner.key();
        state.token_a_vault = ctx.accounts.token_a_vault.key();
        state.token_b_vault = ctx.accounts.token_b_vault.key();
        state.dex1_program = dex1_program;
        state.dex2_program = dex2_program;
        state.min_profit = min_profit;
        Ok(())
    }

    // Check and execute arbitrage
    pub fn execute_arbitrage(ctx: Context<ExecuteArbitrage>, amount: u64) -> Result<()> {
        let state = &ctx.accounts.state;

        // Mock price fetch (in production, use Pyth/Switchboard oracles)
        let dex1_price = get_dex1_price(&ctx.accounts.dex1_program)?; // e.g., SOL/USDC on Raydium
        let dex2_price = get_dex2_price(&ctx.accounts.dex2_program)?; // e.g., SOL/USDC on Orca

        // Calculate arbitrage opportunity with checked arithmetic
        let (buy_dex, sell_dex, profit) = if dex1_price < dex2_price {
            // Buy on DEX1, sell on DEX2
            let price_diff = dex2_price.checked_sub(dex1_price)
                .ok_or(ArbitrageError::ArithmeticOverflow)?;
            let gross_profit = price_diff.checked_mul(amount)
                .ok_or(ArbitrageError::ArithmeticOverflow)?;
            let profit = gross_profit.checked_sub(get_fees(amount))
                .ok_or(ArbitrageError::ArithmeticOverflow)?;
            (&ctx.accounts.dex1_program, &ctx.accounts.dex2_program, profit)
        } else {
            // Buy on DEX2, sell on DEX1
            let price_diff = dex1_price.checked_sub(dex2_price)
                .ok_or(ArbitrageError::ArithmeticOverflow)?;
            let gross_profit = price_diff.checked_mul(amount)
                .ok_or(ArbitrageError::ArithmeticOverflow)?;
            let profit = gross_profit.checked_sub(get_fees(amount))
                .ok_or(ArbitrageError::ArithmeticOverflow)?;
            (&ctx.accounts.dex2_program, &ctx.accounts.dex1_program, profit)
        };

        // Check if profitable
        require!(profit > state.min_profit, ArbitrageError::NotProfitable);

        // Execute swaps (simplified)
        swap_tokens(
            &ctx.accounts.token_a_vault,
            &ctx.accounts.token_b_vault,
            buy_dex,
            amount,
            "buy",
        )?;
        swap_tokens(
            &ctx.accounts.token_b_vault,
            &ctx.accounts.token_a_vault,
            sell_dex,
            amount,
            "sell",
        )?;

        emit!(ArbitrageExecuted {
            profit,
            timestamp: Clock::get()?.unix_timestamp,
        });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = owner, space = 8 + 32 * 5 + 8)]
    pub state: Account<'info, ArbitrageState>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub token_a_vault: Account<'info, TokenAccount>,
    pub token_b_vault: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ExecuteArbitrage<'info> {
    #[account(
        mut,
        has_one = owner,
        has_one = token_a_vault,
        has_one = token_b_vault,
        has_one = dex1_program,
        has_one = dex2_program
    )]
    pub state: Account<'info, ArbitrageState>,
    #[account(mut)]
    pub token_a_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub token_b_vault: Account<'info, TokenAccount>,
    /// CHECK: Validated by has_one constraint on state
    pub dex1_program: AccountInfo<'info>,
    /// CHECK: Validated by has_one constraint on state
    pub dex2_program: AccountInfo<'info>,
    pub owner: Signer<'info>,
}

#[error_code]
pub enum ArbitrageError {
    #[msg("Arbitrage is not profitable")]
    NotProfitable,
    #[msg("Arithmetic overflow or underflow")]
    ArithmeticOverflow,
}

// Mock functions (replace with real DEX/oracle integrations)
fn get_dex1_price(_dex_program: &AccountInfo) -> Result<u64> {
    Ok(100_000_000) // Mock price (e.g., 100 USDC per SOL in lamports)
}

fn get_dex2_price(_dex_program: &AccountInfo) -> Result<u64> {
    Ok(101_000_000) // Mock price (e.g., 101 USDC per SOL in lamports)
}

pub fn get_fees(amount: u64) -> u64 {
    amount / 100 // Mock 1% fee
}

fn swap_tokens<'info>(
    _from_vault: &Account<'info, TokenAccount>,
    _to_vault: &Account<'info, TokenAccount>,
    _dex_program: &AccountInfo<'info>,
    _amount: u64,
    _direction: &str,
) -> Result<()> {
    // Implement DEX swap logic (e.g., call Raydium/Orca program)
    Ok(())
}

#[event]
pub struct ArbitrageExecuted {
    pub profit: u64,
    pub timestamp: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fees_calculation() {
        let amount = 100_000_000;
        let fees = get_fees(amount);
        assert_eq!(fees, 1_000_000); // 1% of amount
    }

    #[test]
    fn test_fees_zero_amount() {
        let amount = 0;
        let fees = get_fees(amount);
        assert_eq!(fees, 0);
    }

    #[test]
    fn test_fee_percentage() {
        let amount = 1_000_000_000; // 1 SOL
        let fees = get_fees(amount);
        assert_eq!(fees, 10_000_000); // 0.01 SOL (1%)
    }
}
