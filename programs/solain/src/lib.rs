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

    // 2. Initialize user profile
    pub fn initialize_profile(
        ctx: Context<InitializeProfile>,
        weight_kg: u16,
        height_cm: u16,
        age: u8,
        gender: u8,
        target_weight_kg: u16,
        weekly_workout_goal: u8,
        daily_calorie_goal: u16,
    ) -> Result<()> {
        instructions::initialize_profile::initialize_profile(
            ctx,
            weight_kg,
            height_cm,
            age,
            gender,
            target_weight_kg,
            weekly_workout_goal,
            daily_calorie_goal,
        )
    }

    // 3. Initialize workout (log workout entry)
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
        weight_lifted: Option<u32>,
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
            weight_lifted,
        )
    }

    // 4. Update workout
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

    // 5. Delete workout
    pub fn delete_workout(ctx: Context<DeleteWorkout>, workout_id: u64) -> Result<()> {
        instructions::delete_workout::delete_workout(ctx, workout_id)
    }

    // 6. Update measurements
    pub fn update_measurements(
        ctx: Context<UpdateMeasurements>,
        weight_kg: Option<u16>,
        chest_cm: Option<u16>,
        waist_cm: Option<u16>,
        hips_cm: Option<u16>,
        thigh_cm: Option<u16>,
        neck_cm: Option<u16>,
        bicep_cm: Option<u16>,
    ) -> Result<()> {
        instructions::update_measurements::update_measurements(
            ctx,
            weight_kg,
            chest_cm,
            waist_cm,
            hips_cm,
            thigh_cm,
            neck_cm,
            bicep_cm,
        )
    }

    // 7. Update goals
    pub fn update_goals(
        ctx: Context<UpdateGoals>,
        target_weight_kg: Option<u16>,
        weekly_workout_goal: Option<u8>,
        daily_calorie_goal: Option<u16>,
    ) -> Result<()> {
        instructions::update_goals::update_goals(
            ctx,
            target_weight_kg,
            weekly_workout_goal,
            daily_calorie_goal,
        )
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
