use crate::errors::WorkoutError;
use crate::states::*;
use anchor_lang::prelude::*;

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
    let config = &mut ctx.accounts.config;

    if workout_id != config.next_workout_id {
        return Err(WorkoutError::InvalidWorkoutId.into());
    }
    if name.as_bytes().len() > WORKOUT_NAME_LENGTH {
        return err!(WorkoutError::WorkoutNameTooLong);
    }
    if category.as_bytes().len() > WORKOUT_CATEGORY_LENGTH {
        return err!(WorkoutError::WorkoutCategoryTooLong);
    }
    if difficulty < 1 || difficulty > 10 {
        return err!(WorkoutError::InvalidDifficulty);
    }

    let workout = &mut ctx.accounts.workout;
    let clock = Clock::get()?;

    // Save workout entry
    workout.workout_id = workout_id;
    workout.name = name;
    workout.reps = reps;
    workout.sets = sets;
    workout.duration_sec = duration_sec;
    workout.calories = calories;
    workout.difficulty = difficulty;
    workout.workout_author = ctx.accounts.workout_authority.key();
    workout.category = category;
    workout.timestamp = clock.unix_timestamp;
    workout.bump = ctx.bumps.workout;

    // Update user profile statistics
    let profile = &mut ctx.accounts.user_profile;
    
    profile.total_workouts = profile.total_workouts.checked_add(1)
        .ok_or(WorkoutError::Overflow)?;
    
    profile.workouts_this_week = profile.workouts_this_week.checked_add(1)
        .ok_or(WorkoutError::Overflow)?;
    
    profile.workouts_this_month = profile.workouts_this_month.checked_add(1)
        .ok_or(WorkoutError::Overflow)?;
    
    // Update personal records
    if calories > profile.max_calories_session {
        profile.max_calories_session = calories;
    }
    
    if let Some(weight) = weight_lifted {
        if weight > profile.heaviest_weight_lifted {
            profile.heaviest_weight_lifted = weight;
            msg!("New PR! Weight lifted: {}g", weight);
        }
    }
    
    profile.last_workout_date = clock.unix_timestamp;
    profile.last_updated = clock.unix_timestamp;

    // Update global config
    config.next_workout_id += 1;
    config.total_workouts += 1;

    msg!(
        "Workout #{} created: {}. Total user workouts: {}",
        workout_id,
        workout.name,
        profile.total_workouts
    );

    Ok(())
}

#[derive(Accounts)]
#[instruction(workout_id: u64)]
pub struct InitializeWorkout<'info> {
    #[account(
        mut,
        seeds = [b"config"],
        bump = config.bump
    )]
    pub config: Account<'info, ProgramConfig>,

    #[account(mut)]
    pub workout_authority: Signer<'info>,

    #[account(
        init,
        payer = workout_authority,
        space = 8 + Workout::INIT_SPACE,
        seeds = [
            b"workout",
            workout_authority.key().as_ref(),
            &workout_id.to_le_bytes()
        ],
        bump
    )]
    pub workout: Account<'info, Workout>,
    
    #[account(
        mut,
        seeds = [
            b"user_profile",
            workout_authority.key().as_ref()
        ],
        bump = user_profile.bump,
        constraint = user_profile.user == workout_authority.key() @ WorkoutError::Unauthorized
    )]
    pub user_profile: Account<'info, UserProfile>,

    pub system_program: Program<'info, System>,
}