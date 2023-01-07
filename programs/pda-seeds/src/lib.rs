use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod pda_seeds {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, id: u8) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(id: u8)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = signer,
        space = 8 + 8,
        seeds= ["seed".as_ref(), signer.key().as_ref(), &id.to_le_bytes()],
        bump
    )]
    pub pda: Account<'info, Data>,
    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct Data {
    pub data: u64,
}
