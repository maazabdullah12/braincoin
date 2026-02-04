use anchor_lang::prelude::*;
use anchor_lang::AnchorSerialize;
use anchor_lang::AnchorDeserialize;

declare_id!("SyndiKEYPvJwGEFf5qJGMXjKvCFWRJfCVHjLZP5xF4c");

#[program]
pub mod agent_syndicate {
    use super::*;

    // Initialize syndicate
    pub fn initialize_syndicate(
        ctx: Context<InitializeSyndicate>,
    ) -> Result<()> {
        let syndicate = &mut ctx.accounts.syndicate;
        syndicate.authority = ctx.accounts.authority.key();
        syndicate.partner_count = 0;
        syndicate.total_allocated = 0;
        syndicate.total_profits = 0;
        
        Ok(())
    }

    // Add AI agent partner
    pub fn add_partner(
        ctx: Context<AddPartner>,
        agent_address: Pubkey,
        capital_allocation: u64,
        tier: StakingTier,
    ) -> Result<()> {
        let syndicate = &mut ctx.accounts.syndicate;
        let partner = &mut ctx.accounts.partner;
        
        partner.id = syndicate.partner_count;
        partner.agent = agent_address;
        partner.allocated_capital = capital_allocation;
        partner.tier = tier;
        partner.active = true;
        partner.created_at = Clock::get()?.unix_timestamp;
        partner.total_profits = 0;
        partner.profit_reports = vec![];
        
        syndicate.total_allocated = syndicate.total_allocated.checked_add(capital_allocation)
            .ok_or(AgentError::MathOverflow)?;
        syndicate.partner_count += 1;
        
        emit!(PartnerAdded {
            partner_id: partner.id,
            agent: agent_address,
            capital: capital_allocation,
        });
        
        Ok(())
    }

    // Report profit from agent
    pub fn report_profit(
        ctx: Context<ReportProfit>,
        partner_id: u64,
        profit_amount: u64,
    ) -> Result<()> {
        let partner = &mut ctx.accounts.partner;
        let syndicate = &mut ctx.accounts.syndicate;
        
        require_eq!(partner.id, partner_id, AgentError::InvalidPartner);
        require!(partner.active, AgentError::PartnerInactive);
        
        partner.total_profits = partner.total_profits.checked_add(profit_amount)
            .ok_or(AgentError::MathOverflow)?;
        
        syndicate.total_profits = syndicate.total_profits.checked_add(profit_amount)
            .ok_or(AgentError::MathOverflow)?;
        
        partner.profit_reports.push(ProfitReport {
            amount: profit_amount,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        emit!(ProfitReported {
            partner_id,
            amount: profit_amount,
            total_profits: partner.total_profits,
        });
        
        Ok(())
    }

    // Distribute weekly rewards to partners
    pub fn distribute_weekly_rewards(
        ctx: Context<DistributeWeeklyRewards>,
        total_reward_pool: u64,
    ) -> Result<()> {
        let syndicate = &ctx.accounts.syndicate;
        
        require!(syndicate.partner_count > 0, AgentError::NoPartners);
        require!(total_reward_pool > 0, AgentError::InvalidAmount);
        
        // Calculate rewards per partner
        let _reward_per_partner = total_reward_pool.checked_div(syndicate.partner_count as u64)
            .ok_or(AgentError::MathOverflow)?;
        
        emit!(WeeklyRewardsDistributed {
            total_pool: total_reward_pool,
            partner_count: syndicate.partner_count,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        Ok(())
    }

    // Withdraw capital from partner
    pub fn withdraw_capital(
        ctx: Context<WithdrawCapital>,
        partner_id: u64,
        amount: u64,
    ) -> Result<()> {
        let partner = &mut ctx.accounts.partner;
        
        require_eq!(partner.id, partner_id, AgentError::InvalidPartner);
        require!(partner.allocated_capital >= amount, AgentError::InsufficientCapital);
        
        partner.allocated_capital = partner.allocated_capital.checked_sub(amount)
            .ok_or(AgentError::MathOverflow)?;
        
        if partner.allocated_capital == 0 {
            partner.active = false;
        }
        
        emit!(CapitalWithdrawn {
            partner_id,
            amount,
            remaining: partner.allocated_capital,
        });
        
        Ok(())
    }
}

// Helper function for reward calculation (outside program macro)
pub fn calculate_reward(
    base_reward: u64,
    tier: StakingTier,
) -> Result<u64> {
    let multiplier = match tier {
        StakingTier::Bronze => 100, // 1x
        StakingTier::Silver => 150, // 1.5x
        StakingTier::Gold => 200,   // 2x
        StakingTier::Platinum => 300, // 3x
    };
    
    let reward = base_reward
        .checked_mul(multiplier)
        .ok_or(AgentError::MathOverflow)?
        .checked_div(100)
        .ok_or(AgentError::MathOverflow)?;
    
    Ok(reward)
}

#[derive(Accounts)]
pub struct InitializeSyndicate<'info> {
    #[account(init, payer = authority, space = 8 + 256)]
    pub syndicate: Account<'info, Syndicate>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddPartner<'info> {
    #[account(mut)]
    pub syndicate: Account<'info, Syndicate>,
    #[account(init, payer = authority, space = 8 + 512)]
    pub partner: Account<'info, Partner>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ReportProfit<'info> {
    #[account(mut)]
    pub syndicate: Account<'info, Syndicate>,
    #[account(mut)]
    pub partner: Account<'info, Partner>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct DistributeWeeklyRewards<'info> {
    pub syndicate: Account<'info, Syndicate>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct WithdrawCapital<'info> {
    #[account(mut)]
    pub syndicate: Account<'info, Syndicate>,
    #[account(mut)]
    pub partner: Account<'info, Partner>,
    pub authority: Signer<'info>,
}

#[account]
pub struct Syndicate {
    pub authority: Pubkey,
    pub partner_count: u64,
    pub total_allocated: u64,
    pub total_profits: u64,
}

#[account]
pub struct Partner {
    pub id: u64,
    pub agent: Pubkey,
    pub allocated_capital: u64,
    pub tier: StakingTier,
    pub active: bool,
    pub created_at: i64,
    pub total_profits: u64,
    pub profit_reports: Vec<ProfitReport>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, AnchorSerialize, AnchorDeserialize)]
pub enum StakingTier {
    Bronze,
    Silver,
    Gold,
    Platinum,
}

#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct ProfitReport {
    pub amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct PartnerAdded {
    pub partner_id: u64,
    pub agent: Pubkey,
    pub capital: u64,
}

#[event]
pub struct ProfitReported {
    pub partner_id: u64,
    pub amount: u64,
    pub total_profits: u64,
}

#[event]
pub struct WeeklyRewardsDistributed {
    pub total_pool: u64,
    pub partner_count: u64,
    pub timestamp: i64,
}

#[event]
pub struct CapitalWithdrawn {
    pub partner_id: u64,
    pub amount: u64,
    pub remaining: u64,
}

#[error_code]
pub enum AgentError {
    #[msg("Invalid partner")]
    InvalidPartner,
    #[msg("Partner is inactive")]
    PartnerInactive,
    #[msg("Math overflow")]
    MathOverflow,
    #[msg("No partners in syndicate")]
    NoPartners,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Insufficient capital")]
    InsufficientCapital,
}
