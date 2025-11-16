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

    // 1. Initialize config
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
    pub fn initialize_workout(
        ctx: Context<InitializeWorkout>,
        workout_id: u64,
        name: String,
        reps: u16,
        sets: u8,
        duration_sec: u32,
        calories: u16,
        difficulty: u8,
        category: String,
    ) -> Result<()> {
        instructions::initialize_workout::initialize_workout(
            ctx,
            workout_id,
            name,
            reps,
            sets,
            duration_sec,
            calories,
            difficulty,
            category,
        )
    }

    // 3. Update workout
    pub fn update_workout(
        ctx: Context<UpdateWorkout>,
        workout_id: u64,
        name: Option<String>,
        reps: Option<u16>,
        sets: Option<u8>,
        duration_sec: Option<u32>,
        calories: Option<u16>,
        difficulty: Option<u8>,
        category: Option<String>,
    ) -> Result<()> {
        instructions::update_workout::update_workout(
            ctx,
            workout_id,
            name,
            reps,
            sets,
            duration_sec,
            calories,
            difficulty,
            category,
        )
    }

    // 4. Delete workout
    pub fn delete_workout(ctx: Context<DeleteWorkout>, workout_id: u64) -> Result<()> {
        instructions::delete_workout::delete_workout(ctx, workout_id)
    }
}

// Initialize context залишається в lib.rs
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
