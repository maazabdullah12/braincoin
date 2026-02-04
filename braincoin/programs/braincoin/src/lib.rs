use anchor_lang::prelude::*;

declare_id!("3Jdomepj9sSwumtnzgQ5SLhacZfNBshptys94Qpnwtnd");

#[program]
pub mod braincoin {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
