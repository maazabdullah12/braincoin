use anchor_lang::prelude::*;
use anchor_lang::AnchorSerialize;
use anchor_lang::AnchorDeserialize;
use anchor_spl::token::Mint;

declare_id!("9B5X5wUGD8PJwTCSyNFAKnwKwq4qpi3JJcqywikaHnnR");

#[program]
pub mod brain {
    use super::*;

    // Initialize the brain state
    pub fn initialize_brain(
        ctx: Context<InitializeBrain>,
        pnl_limit: i64,
    ) -> Result<()> {
        let brain_state = &mut ctx.accounts.brain_state;
        brain_state.mint = ctx.accounts.mint.key();
        brain_state.authority = ctx.accounts.authority.key();
        brain_state.pnl_balance = 0;
        brain_state.frozen = false;
        brain_state.evolution_tier = EvolutionTier::Cellular;
        brain_state.pnl_limit = pnl_limit;
        brain_state.last_updated = Clock::get()?.unix_timestamp;
        
        Ok(())
    }

    // Report PnL and check for circuit breaker
    pub fn report_pnl(
        ctx: Context<ReportPnL>,
        pnl_amount: i64,
    ) -> Result<()> {
        let brain_state = &mut ctx.accounts.brain_state;
        
        require!(!brain_state.frozen, BrainError::BrainFrozen);
        
        brain_state.pnl_balance = brain_state.pnl_balance.checked_add(pnl_amount)
            .ok_or(BrainError::MathOverflow)?;
        
        // Check circuit breaker: -30% threshold
        if brain_state.pnl_balance < brain_state.pnl_limit {
            brain_state.frozen = true;
            emit!(CircuitBreakerTriggered {
                pnl_balance: brain_state.pnl_balance,
                timestamp: Clock::get()?.unix_timestamp,
            });
        }
        
        brain_state.last_updated = Clock::get()?.unix_timestamp;
        
        Ok(())
    }

    // Propose partnership with another AI agent
    pub fn propose_partnership(
        ctx: Context<ProposePartnership>,
        agent_address: Pubkey,
        capital_allocation: u64,
    ) -> Result<()> {
        let brain_state = &ctx.accounts.brain_state;
        require!(!brain_state.frozen, BrainError::BrainFrozen);
        
        let partnership = &mut ctx.accounts.partnership;
        partnership.agent = agent_address;
        partnership.allocated_capital = capital_allocation;
        partnership.active = true;
        partnership.created_at = Clock::get()?.unix_timestamp;
        
        emit!(PartnershipProposed {
            agent: agent_address,
            capital: capital_allocation,
        });
        
        Ok(())
    }

    // Evolve to next tier
    pub fn evolve_tier(ctx: Context<EvolveTier>) -> Result<()> {
        let brain_state = &mut ctx.accounts.brain_state;
        
        let next_tier = match brain_state.evolution_tier {
            EvolutionTier::Cellular => EvolutionTier::Sentient,
            EvolutionTier::Sentient => EvolutionTier::Superintelligent,
            EvolutionTier::Superintelligent => EvolutionTier::Godlike,
            EvolutionTier::Godlike => EvolutionTier::Godlike, // Already at max
        };
        
        brain_state.evolution_tier = next_tier;
        
        emit!(TierEvolved {
            new_tier: format!("{:?}", next_tier),
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        Ok(())
    }

    // Unfreeze brain (emergency admin function)
    pub fn unfreeze_brain(ctx: Context<UnfreezeBrain>) -> Result<()> {
        let brain_state = &mut ctx.accounts.brain_state;
        brain_state.frozen = false;
        
        emit!(BrainUnfrozen {
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeBrain<'info> {
    #[account(init, payer = authority, space = 8 + 256)]
    pub brain_state: Account<'info, BrainState>,
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ReportPnL<'info> {
    #[account(mut)]
    pub brain_state: Account<'info, BrainState>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ProposePartnership<'info> {
    #[account(mut)]
    pub brain_state: Account<'info, BrainState>,
    #[account(init, payer = authority, space = 8 + 256)]
    pub partnership: Account<'info, Partnership>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EvolveTier<'info> {
    #[account(mut)]
    pub brain_state: Account<'info, BrainState>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct UnfreezeBrain<'info> {
    #[account(mut)]
    pub brain_state: Account<'info, BrainState>,
    pub authority: Signer<'info>,
}

#[account]
pub struct BrainState {
    pub mint: Pubkey,
    pub authority: Pubkey,
    pub pnl_balance: i64,
    pub pnl_limit: i64,
    pub frozen: bool,
    pub evolution_tier: EvolutionTier,
    pub last_updated: i64,
}

#[account]
pub struct Partnership {
    pub agent: Pubkey,
    pub allocated_capital: u64,
    pub active: bool,
    pub created_at: i64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, AnchorSerialize, AnchorDeserialize)]
pub enum EvolutionTier {
    Cellular,
    Sentient,
    Superintelligent,
    Godlike,
}

#[event]
pub struct CircuitBreakerTriggered {
    pub pnl_balance: i64,
    pub timestamp: i64,
}

#[event]
pub struct PartnershipProposed {
    pub agent: Pubkey,
    pub capital: u64,
}

#[event]
pub struct TierEvolved {
    pub new_tier: String,
    pub timestamp: i64,
}

#[event]
pub struct BrainUnfrozen {
    pub timestamp: i64,
}

#[error_code]
pub enum BrainError {
    #[msg("Brain is frozen by circuit breaker")]
    BrainFrozen,
    #[msg("Math overflow occurred")]
    MathOverflow,
}
