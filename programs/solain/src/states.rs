use anchor_lang::prelude::*;

pub const WORKOUT_NAME_LENGTH: usize = 16;
pub const WORKOUT_CATEGORY_LENGTH: usize = 10;

#[account]
#[derive(InitSpace)]
pub struct Workout {
    pub workout_id: u64,        // Workout id
    pub workout_author: Pubkey, // Author
    #[max_len(WORKOUT_NAME_LENGTH)]
    pub name: String, // Name
    pub reps: u16,              // Reps count
    pub sets: u8,               // Sets
    pub duration_sec: u32,      // Duration time
    pub calories: u16,          // Calories burned
    pub difficulty: u8,         // Difficulty level from 1 to 10
    #[max_len(WORKOUT_CATEGORY_LENGTH)]
    pub category: String, // type ("legs", "cardio", "push", etc.)
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct ProgramConfig {
    pub admin: Pubkey,
    pub next_workout_id: u64,
    pub total_workouts: u64,
    pub paused: bool,
    pub bump: u8,
}
