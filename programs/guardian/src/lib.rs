use anchor_lang::prelude::*;
use anchor_lang::AnchorSerialize;
use anchor_lang::AnchorDeserialize;

declare_id!("GuarKEYWJHwVbSRhPLy2dVMVS8zzc5KxzDXVjSTWZfZV");

#[program]
pub mod guardian {
    use super::*;

    // Initialize governance
    pub fn initialize_governance(
        ctx: Context<InitializeGovernance>,
        signer_1: Pubkey,
        signer_2: Pubkey,
        signer_3: Pubkey,
    ) -> Result<()> {
        let governance = &mut ctx.accounts.governance;
        governance.signer_1 = signer_1;
        governance.signer_2 = signer_2;
        governance.signer_3 = signer_3;
        governance.proposal_count = 0;
        
        Ok(())
    }

    // Submit proposal
    pub fn submit_proposal(
        ctx: Context<SubmitProposal>,
        title: String,
        description: String,
        proposal_type: ProposalType,
    ) -> Result<()> {
        let governance = &mut ctx.accounts.governance;
        let proposal = &mut ctx.accounts.proposal;
        
        proposal.id = governance.proposal_count;
        proposal.proposer = ctx.accounts.proposer.key();
        proposal.title = title;
        proposal.description = description;
        proposal.proposal_type = proposal_type;
        proposal.created_at = Clock::get()?.unix_timestamp;
        proposal.timelock_until = Clock::get()?.unix_timestamp + 48 * 3600; // 48 hours
        proposal.status = ProposalStatus::Pending;
        proposal.approved_by = vec![];
        proposal.rejected = false;
        
        governance.proposal_count += 1;
        
        emit!(ProposalSubmitted {
            proposal_id: proposal.id,
            proposer: proposal.proposer,
            timestamp: proposal.created_at,
        });
        
        Ok(())
    }

    // Approve proposal (multi-sig)
    pub fn approve_proposal(
        ctx: Context<ApproveProposal>,
        proposal_id: u64,
    ) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        let governance = &ctx.accounts.governance;
        
        require_eq!(proposal.id, proposal_id, GovernanceError::InvalidProposal);
        require!(proposal.status == ProposalStatus::Pending, GovernanceError::ProposalNotPending);
        
        // Check if signer is one of the governance signers
        let signer = ctx.accounts.signer.key();
        let is_valid_signer = signer == governance.signer_1 
            || signer == governance.signer_2 
            || signer == governance.signer_3;
        
        require!(is_valid_signer, GovernanceError::InvalidSigner);
        require!(!proposal.approved_by.contains(&signer), GovernanceError::AlreadyApproved);
        
        // Check timelock
        let current_time = Clock::get()?.unix_timestamp;
        require!(current_time >= proposal.timelock_until, GovernanceError::TimelockedProposal);
        
        proposal.approved_by.push(signer);
        
        // Approve if 2 out of 3 signers approve (2-of-3 multisig)
        if proposal.approved_by.len() >= 2 {
            proposal.status = ProposalStatus::Approved;
            
            emit!(ProposalApproved {
                proposal_id: proposal.id,
                approvers: proposal.approved_by.clone(),
                timestamp: current_time,
            });
        }
        
        Ok(())
    }

    // Reject proposal
    pub fn reject_proposal(
        ctx: Context<RejectProposal>,
        proposal_id: u64,
    ) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        let governance = &ctx.accounts.governance;
        
        require_eq!(proposal.id, proposal_id, GovernanceError::InvalidProposal);
        require!(proposal.status == ProposalStatus::Pending, GovernanceError::ProposalNotPending);
        
        let signer = ctx.accounts.signer.key();
        let is_valid_signer = signer == governance.signer_1 
            || signer == governance.signer_2 
            || signer == governance.signer_3;
        
        require!(is_valid_signer, GovernanceError::InvalidSigner);
        
        proposal.status = ProposalStatus::Rejected;
        proposal.rejected = true;
        
        emit!(ProposalRejected {
            proposal_id: proposal.id,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        Ok(())
    }

    // Distribute rewards to holders
    pub fn distribute_rewards(
        ctx: Context<DistributeRewards>,
        reward_amount: u64,
    ) -> Result<()> {
        let governance = &ctx.accounts.governance;
        
        // Check if proposal authorizes this action
        require!(governance.proposal_count > 0, GovernanceError::NoProposals);
        
        emit!(RewardsDistributed {
            amount: reward_amount,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeGovernance<'info> {
    #[account(init, payer = authority, space = 8 + 256)]
    pub governance: Account<'info, Governance>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SubmitProposal<'info> {
    #[account(mut)]
    pub governance: Account<'info, Governance>,
    #[account(init, payer = proposer, space = 8 + 512)]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub proposer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ApproveProposal<'info> {
    pub governance: Account<'info, Governance>,
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct RejectProposal<'info> {
    pub governance: Account<'info, Governance>,
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct DistributeRewards<'info> {
    pub governance: Account<'info, Governance>,
    pub authority: Signer<'info>,
}

#[account]
pub struct Governance {
    pub signer_1: Pubkey,
    pub signer_2: Pubkey,
    pub signer_3: Pubkey,
    pub proposal_count: u64,
}

#[account]
pub struct Proposal {
    pub id: u64,
    pub proposer: Pubkey,
    pub title: String,
    pub description: String,
    pub proposal_type: ProposalType,
    pub created_at: i64,
    pub timelock_until: i64,
    pub status: ProposalStatus,
    pub approved_by: Vec<Pubkey>,
    pub rejected: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, AnchorSerialize, AnchorDeserialize)]
pub enum ProposalType {
    PartnershipApproval,
    TreasuryWithdrawal,
    ParameterChange,
    EmergencyAction,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, AnchorSerialize, AnchorDeserialize)]
pub enum ProposalStatus {
    Pending,
    Approved,
    Rejected,
    Executed,
}

#[event]
pub struct ProposalSubmitted {
    pub proposal_id: u64,
    pub proposer: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct ProposalApproved {
    pub proposal_id: u64,
    pub approvers: Vec<Pubkey>,
    pub timestamp: i64,
}

#[event]
pub struct ProposalRejected {
    pub proposal_id: u64,
    pub timestamp: i64,
}

#[event]
pub struct RewardsDistributed {
    pub amount: u64,
    pub timestamp: i64,
}

#[error_code]
pub enum GovernanceError {
    #[msg("Invalid proposal")]
    InvalidProposal,
    #[msg("Proposal is not pending")]
    ProposalNotPending,
    #[msg("Invalid signer")]
    InvalidSigner,
    #[msg("Already approved")]
    AlreadyApproved,
    #[msg("Proposal is still timelocked")]
    TimelockedProposal,
    #[msg("No proposals exist")]
    NoProposals,
}
