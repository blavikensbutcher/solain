use crate::states::*;
use anchor_lang::prelude::*;

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
    let profile = &mut ctx.accounts.user_profile;
    let clock = Clock::get()?;

    // Basic info
    profile.user = ctx.accounts.user.key();
    profile.weight_kg = weight_kg;
    profile.height_cm = height_cm;
    profile.age = age;
    profile.gender = gender;

    // Body measurements - initialize to 0
    profile.chest_cm = 0;
    profile.waist_cm = 0;
    profile.hips_cm = 0;
    profile.thigh_cm = 0;
    profile.neck_cm = 0;
    profile.bicep_cm = 0;

    // Performance tracking
    profile.workouts_this_week = 0;
    profile.workouts_this_month = 0;

    // Personal records
    profile.heaviest_weight_lifted = 0;
    profile.max_calories_session = 0;

    // Goals
    profile.target_weight_kg = target_weight_kg;
    profile.weekly_workout_goal = weekly_workout_goal;
    profile.daily_calorie_goal = daily_calorie_goal;

    // Timestamps
    profile.last_workout_date = 0;
    profile.profile_created = clock.unix_timestamp;
    profile.last_updated = clock.unix_timestamp;
    profile.bump = ctx.bumps.user_profile;

    msg!("User profile created for: {:?}", profile.user);
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeProfile<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + UserProfile::INIT_SPACE,
        seeds = [
            b"user_profile",
            user.key().as_ref()
        ],
        bump
    )]
    pub user_profile: Account<'info, UserProfile>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}
