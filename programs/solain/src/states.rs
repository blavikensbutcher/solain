use anchor_lang::prelude::*;

pub const WORKOUT_NAME_LENGTH: usize = 64; 
pub const WORKOUT_CATEGORY_LENGTH: usize = 20;

#[account]
#[derive(InitSpace)]
pub struct Workout {
    pub workout_id: u64,
    pub workout_author: Pubkey,
    #[max_len(64)]
    pub name: String,
    pub reps: u16,
    pub sets: u8,
    pub duration_sec: u32,
    pub calories: u16,
    pub difficulty: u8,
    #[max_len(20)]
    pub category: String,
    pub bump: u8,
    pub timestamp: i64,
    pub weight_lifted: Option<u32>,
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

#[account]
#[derive(InitSpace)]
pub struct UserProfile {
    // Basic info - source of truth
    pub user: Pubkey,
    pub weight_kg: u16,
    pub height_cm: u16,
    pub age: u8,
    pub gender: u8,

    // Body measurements in cm - source of truth
    pub chest_cm: u16,
    pub waist_cm: u16,
    pub hips_cm: u16,
    pub thigh_cm: u16,
    pub neck_cm: u16,
    pub bicep_cm: u16,

    // Performance tracking
    pub workouts_this_week: u8,
    pub workouts_this_month: u16,
    pub total_workouts: u32,

    // Personal records
    pub heaviest_weight_lifted: u32, // in grams
    pub max_calories_session: u16,

    // Goals
    pub target_weight_kg: u16,
    pub weekly_workout_goal: u8,
    pub daily_calorie_goal: u16,

    // Timestamps
    pub last_workout_date: i64,
    pub profile_created: i64,
    pub last_updated: i64,
    pub bump: u8,
}
