use crate::{errors::WorkoutError, states::*};
use anchor_lang::prelude::*;

pub fn delete_workout(ctx: Context<DeleteWorkout>, workout_id: u64) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
#[instruction(workout_id: u64)]
pub struct DeleteWorkout<'info> {
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
        close = workout_author,
        has_one = workout_author @ WorkoutError::Unauthorized,
    )]
    pub workout: Account<'info, Workout>,
}