use anchor_lang::prelude::*;

use crate::errors::WorkoutError;
use crate::states::*;

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
    if name.as_bytes().len() > WORKOUT_NAME_LENGTH {
        return err!(WorkoutError::WorkoutNameTooLong);
    }

    if category.as_bytes().len() > WORKOUT_CATEGORY_LENGTH {
        return err!(WorkoutError::WorkoutCategoryTooLong);
    }

    if difficulty < 1 || difficulty > 10 {
        return err!(WorkoutError::InvalidDifficulty);
    }

    let workout: &mut Account<'_, Workout> = &mut ctx.accounts.workout;

    workout.workout_id = workout_id;
    workout.name = name;
    workout.reps = reps;
    workout.sets = sets;
    workout.duration_sec = duration_sec;
    workout.calories = calories;
    workout.difficulty = difficulty;
    workout.workout_author = ctx.accounts.workout_authority.key();
    workout.category = category;
    workout.bump = ctx.bumps.workout;

    Ok(())
}

#[derive(Accounts)]
#[instruction(workout_id: u64)]
pub struct InitializeWorkout<'info> {
    #[account(mut)]
    pub workout_authority: Signer<'info>,
    #[account(
        init,
        payer = workout_authority,
        space = 8 + Workout::INIT_SPACE,
        seeds = [
            b"workout", 
            workout_authority.key().as_ref(),
            &workout_id.to_le_bytes()],
        bump
     )]
    pub workout: Account<'info, Workout>,
    pub system_program: Program<'info, System>,
}
