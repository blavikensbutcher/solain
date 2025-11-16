use anchor_lang::prelude::*;

pub const WORKOUT_NAME_LENGTH: usize = 16;
pub const WORKOUT_CATEGORY_LENGTH: usize = 10;

#[account]
#[derive(InitSpace)]
pub struct Workout {
    pub workout_id: u64,
    pub workout_author: Pubkey,
    #[max_len(32)]
    pub name: String,
    pub reps: u16,
    pub sets: u8,
    pub duration_sec: u32,
    pub calories: u16,
    pub difficulty: u8,
    #[max_len(20)]
    pub category: String,
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
