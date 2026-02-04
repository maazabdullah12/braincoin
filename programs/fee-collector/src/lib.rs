use anchor_lang::prelude::*;

declare_id!("FeesKEYLrJ1A2nJHPp2Wd6AqMHYHKrzWQ7ZXx5cJjQH");

#[program]
pub mod fee_collector {
    use super::*;

    // Initialize fee collector
    pub fn initialize_fee_collector(
        ctx: Context<InitializeFeeCollector>,
        treasury_wallet: Pubkey,
        fee_percentage: u64,
    ) -> Result<()> {
        let fee_collector = &mut ctx.accounts.fee_collector;
        fee_collector.authority = ctx.accounts.authority.key();
        fee_collector.treasury_wallet = treasury_wallet;
        fee_collector.fee_percentage = fee_percentage; // e.g., 3-5%
        fee_collector.total_fees_collected = 0;
        fee_collector.total_withdrawn = 0;
        
        Ok(())
    }

    // Record trading fee
    pub fn record_fee(
        ctx: Context<RecordFee>,
        trade_amount: u64,
    ) -> Result<()> {
        let fee_collector = &mut ctx.accounts.fee_collector;
        
        // Calculate fee: trade_amount * fee_percentage / 100
        let fee_amount = trade_amount
            .checked_mul(fee_collector.fee_percentage)
            .ok_or(FeeError::MathOverflow)?
            .checked_div(100)
            .ok_or(FeeError::MathOverflow)?;
        
        fee_collector.total_fees_collected = fee_collector.total_fees_collected
            .checked_add(fee_amount)
            .ok_or(FeeError::MathOverflow)?;
        
        emit!(FeeRecorded {
            trade_amount,
            fee_amount,
            total_collected: fee_collector.total_fees_collected,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        Ok(())
    }

    // Withdraw accumulated fees to treasury
    pub fn withdraw_fees(
        ctx: Context<WithdrawFees>,
        amount: u64,
    ) -> Result<()> {
        let fee_collector = &mut ctx.accounts.fee_collector;
        
        require!(
            fee_collector.total_fees_collected >= amount,
            FeeError::InsufficientFees
        );
        
        fee_collector.total_fees_collected = fee_collector.total_fees_collected
            .checked_sub(amount)
            .ok_or(FeeError::MathOverflow)?;
        
        fee_collector.total_withdrawn = fee_collector.total_withdrawn
            .checked_add(amount)
            .ok_or(FeeError::MathOverflow)?;
        
        emit!(FeesWithdrawn {
            amount,
            timestamp: Clock::get()?.unix_timestamp,
            treasury: fee_collector.treasury_wallet,
        });
        
        Ok(())
    }

    // Update fee percentage
    pub fn update_fee_percentage(
        ctx: Context<UpdateFeePercentage>,
        new_percentage: u64,
    ) -> Result<()> {
        let fee_collector = &mut ctx.accounts.fee_collector;
        
        require!(new_percentage > 0 && new_percentage <= 100, FeeError::InvalidPercentage);
        
        let old_percentage = fee_collector.fee_percentage;
        fee_collector.fee_percentage = new_percentage;
        
        emit!(FeePercentageUpdated {
            old_percentage,
            new_percentage,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        Ok(())
    }

    // Get current fee balance
    pub fn get_fee_balance(ctx: Context<GetFeeBalance>) -> Result<u64> {
        let fee_collector = &ctx.accounts.fee_collector;
        Ok(fee_collector.total_fees_collected)
    }
}

#[derive(Accounts)]
pub struct InitializeFeeCollector<'info> {
    #[account(init, payer = authority, space = 8 + 256)]
    pub fee_collector: Account<'info, FeeCollector>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RecordFee<'info> {
    #[account(mut)]
    pub fee_collector: Account<'info, FeeCollector>,
}

#[derive(Accounts)]
pub struct WithdrawFees<'info> {
    #[account(mut)]
    pub fee_collector: Account<'info, FeeCollector>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateFeePercentage<'info> {
    #[account(mut)]
    pub fee_collector: Account<'info, FeeCollector>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetFeeBalance<'info> {
    pub fee_collector: Account<'info, FeeCollector>,
}

#[account]
pub struct FeeCollector {
    pub authority: Pubkey,
    pub treasury_wallet: Pubkey,
    pub fee_percentage: u64,
    pub total_fees_collected: u64,
    pub total_withdrawn: u64,
}

#[event]
pub struct FeeRecorded {
    pub trade_amount: u64,
    pub fee_amount: u64,
    pub total_collected: u64,
    pub timestamp: i64,
}

#[event]
pub struct FeesWithdrawn {
    pub amount: u64,
    pub timestamp: i64,
    pub treasury: Pubkey,
}

#[event]
pub struct FeePercentageUpdated {
    pub old_percentage: u64,
    pub new_percentage: u64,
    pub timestamp: i64,
}

#[error_code]
pub enum FeeError {
    #[msg("Insufficient fees collected")]
    InsufficientFees,
    #[msg("Math overflow")]
    MathOverflow,
    #[msg("Invalid percentage")]
    InvalidPercentage,
}
