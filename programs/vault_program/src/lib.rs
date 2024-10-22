use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer, transfer};
declare_id!("4B4n1NxmqPy5muRbQYafVotCnS4pg4Wh9hb1TDxqkfAW");

#[program]
pub mod vault_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.state.vault_bump = *ctx.bumps.get("vault").unwrap();
        ctx.accounts.state.state_bump = *ctx.bumps.get("state").unwrap();
        Ok(())
    }
    
  
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        seeds=[b"state", signer.key().as_ref()],
        bump,
        space = VaultState::LEN
    )]
    pub state: Account<'info, VaultState>,
    #[account(
        seeds = [b"vault",state.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
    pub system_program : Program<'info , System>
}


#[derive(Accounts)]
pub struct Payment<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds=[b"vault", state.key().as_ref()],
        bump=state.vault_bump
    )]
    pub vault: SystemAccount<'info>,

    #[account(  seeds=[b"state", signer.key().as_ref()],
    bump=state.state_bump)]
    pub state: Account<'info, VaultState>,
    pub system_program: Program<'info, System>
}

#[account]
pub struct VaultState{
    pub vault_bump: u8,
    pub state_bump: u8
}
impl VaultState {
    const LEN: usize = 8 + 1 + 1;
}