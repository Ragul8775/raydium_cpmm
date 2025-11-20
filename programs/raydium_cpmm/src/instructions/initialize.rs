use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::Token,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use raydium_cpmm_cpi::{
    AmmConfig, cpi, program::RaydiumCpmm, states::{POOL_SEED}
};

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub cp_swap_program: Program<'info,RaydiumCpmm>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub amm_config: Box<Account<'info, AmmConfig>>,

    #[account(
        seeds = [
            raydium_cpmm_cpi::AUTH_SEED.as_bytes(),
        ],
        seeds::program = cp_swap_program.key(),
        bump,
    )]
    pub authority: UncheckedAccount<'info>,

    ///CHECK: Initialize an Account
    #[account(mut,
    seeds=[
        POOL_SEED.as_bytes(),
        amm_config.key().as_ref(),
        token_0_mint.key().as_ref(),
        token_1_mint.key().as_ref(),
    ],
seeds::program = cp_swap_program.key(),
bump)]
pub pool_state : UncheckedAccount<'info>,

#[account(mut,
constraint = token_0_mint.key() < token_1_mint.key(),
mint::token_program = token_0_program,)]
pub token_0_mint:Box<InterfaceAccount<'info, Mint>>,
#[account(
        mint::token_program = token_1_program,
    )]
    pub token_1_mint: Box<InterfaceAccount<'info, Mint>>,
}