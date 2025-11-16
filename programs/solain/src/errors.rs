use anchor_lang::prelude::*;

#[error_code]
pub enum WorkoutError {
    #[msg("Cannot initialize, workout name too long")]
    WorkoutNameTooLong,
    #[msg("Cannot initialize, workout category too long")]
    WorkoutCategoryTooLong,
    #[msg("You are not authorized to modify this workout")]
    Unauthorized,
    #[msg("Difficulty should be between 1 and 10")]
    InvalidDifficulty,
    #[msg("Invalid workout ID")]
    InvalidWorkoutId,
}
