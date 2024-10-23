use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer, transfer};
declare_id!("4B4n1NxmqPy5muRbQYafVotCnS4pg4Wh9hb1TDxqkfAW");

#[program]
pub mod vault_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)?;

        Ok(())
    }
    
    pub fn deposit (ctx: Context<Payment>, amount:u64)  -> Result<()> {

        let transfer_accounts = Transfer {
            from:ctx.accounts.signer.to_account_info(),
            to:ctx.accounts.vault.to_account_info()
        };
        let transfer_ctx = CpiContext::new( ctx.accounts.system_program.to_account_info(), 
        transfer_accounts);

        transfer(transfer_ctx, amount)

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
        space = VaultState::INIT_SPACE,

    )]
    pub state: Account<'info, VaultState>,
    #[account(
        seeds = [b"vault",state.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
    pub system_program : Program<'info , System>
}

impl <'info> Initialize<'info> {
    
pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
    self.state.vault_bump = bumps.vault;
    self.state.state_bump = bumps.state;
    Ok(())
}
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

impl<'info> Payment<'info>  {
    pub fn deposit (ctx: Context<Payment>, amount:u64)  -> Result<()> {

        let transfer_accounts = Transfer {
            from:ctx.accounts.signer.to_account_info(),
            to:ctx.accounts.vault.to_account_info()
        };
        let transfer_ctx = CpiContext::new( ctx.accounts.system_program.to_account_info(), 
        transfer_accounts);

        transfer(transfer_ctx, amount)

    }
  
    
}

impl <'info> Payment<'info> {

    pub fn withdraw(ctx: Context<Payment>, amount:u64) -> Result<()>  {
let transfer_account = Transfer{
    from:ctx.accounts.vault.to_account_info(),
    to:ctx.accounts.signer.to_account_info()
};

let seeds = &[
    b"vault",
    ctx.accounts.state.to_account_info().key.as_ref(),
    &[ctx.accounts.state.vault_bump],
];


let pda_signer = &[&seeds[..]];

let transfer_ctx = CpiContext::new_with_signer(
    ctx.accounts.system_program.to_account_info(),
    transfer_account,
    pda_signer,
);

transfer(transfer_ctx, amount)
    }
    
}

#[account]
pub struct VaultState{
    pub vault_bump: u8,
    pub state_bump: u8
}
impl Space for VaultState {
    const INIT_SPACE: usize = 8 + 1 + 1;
}