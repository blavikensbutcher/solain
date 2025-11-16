use crate::{errors::WorkoutError, states::*};
use anchor_lang::prelude::*;

pub fn update_workout(
    ctx: Context<UpdateWorkout>,
    _workout_id: u64,
    name: Option<String>,
    reps: Option<u16>,
    sets: Option<u8>,
    duration_sec: Option<u32>,
    calories: Option<u16>,
    difficulty: Option<u8>,
    category: Option<String>,
) -> Result<()> {
    let workout = &mut ctx.accounts.workout;

    if let Some(n) = name {
        if n.as_bytes().len() > WORKOUT_NAME_LENGTH {
            return err!(WorkoutError::WorkoutNameTooLong);
        }
        workout.name = n;
    }

    if let Some(c) = category {
        if c.as_bytes().len() > WORKOUT_CATEGORY_LENGTH {
            return err!(WorkoutError::WorkoutCategoryTooLong);
        }
        workout.category = c;
    }

    if let Some(v) = reps {
        workout.reps = v
    }
    if let Some(v) = sets {
        workout.sets = v
    }
    if let Some(v) = duration_sec {
        workout.duration_sec = v
    }
    if let Some(v) = calories {
        workout.calories = v
    }
    if let Some(v) = difficulty {
        workout.difficulty = v
    }

    Ok(())
}

#[derive(Accounts)]
#[instruction(workout_id: u64)]
pub struct UpdateWorkout<'info> {
    #[account(mut)]
    pub workout_author: Signer<'info>,

    #[account(
        mut,
        seeds = [
            b"workout",
            workout_author.key().as_ref(),
            &workout_id.to_le_bytes(),
        ],
        bump = workout.bump,
        has_one = workout_author @ WorkoutError::Unauthorized,
    )]
    pub workout: Account<'info, Workout>,
}
