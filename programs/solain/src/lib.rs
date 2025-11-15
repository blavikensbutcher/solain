use anchor_lang::prelude::*;

declare_id!("2BqFVR96CLqZ6AHue5FbUCXFk4zdiASaoL97wND53BT3");

pub mod errors;
pub mod instructions;
pub mod states;

pub use errors::*;
pub use instructions::*;
pub use states::*;

#[program]
pub mod solain {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.admin = ctx.accounts.authority.key();
        config.next_workout_id = 1;
        config.total_workouts = 0;
        config.paused = false;
        config.bump = ctx.bumps.config;

        msg!("Program initialized by: {:?}", config.admin);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + ProgramConfig::INIT_SPACE,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, ProgramConfig>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}
