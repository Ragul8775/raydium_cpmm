use anchor_lang::prelude::*;

pub mod instructions;
use instructions::*;

declare_id!("Dk8kkDigMizHgCygSmaduSGUoz7fPqqYG7hCpwcMo8tu");

#[program]
pub mod raydium_cpmm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
