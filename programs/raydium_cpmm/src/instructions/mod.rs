use anchor_lang::prelude::*;
use raydium_cpmm_cpi::{
    cpi,
    program::RaydiumCpmm,
};

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub cp_swap_program: Program<'info,RaydiumCpmm>,
    #[account(mut)]
    pub creator: Signer<'info>,
}