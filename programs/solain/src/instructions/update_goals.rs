use crate::states::*;
use anchor_lang::prelude::*;

pub fn update_goals(
    ctx: Context<UpdateGoals>,
    target_weight_kg: Option<u16>,
    weekly_workout_goal: Option<u8>,
    daily_calorie_goal: Option<u16>,
) -> Result<()> {
    let profile = &mut ctx.accounts.user_profile;

    if let Some(target) = target_weight_kg {
        profile.target_weight_kg = target;
    }
    if let Some(weekly_goal) = weekly_workout_goal {
        profile.weekly_workout_goal = weekly_goal;
    }
    if let Some(calorie_goal) = daily_calorie_goal {
        profile.daily_calorie_goal = calorie_goal;
    }

    profile.last_updated = Clock::get()?.unix_timestamp;

    msg!("Goals updated for: {:?}", profile.user);
    Ok(())
}

#[derive(Accounts)]
pub struct UpdateGoals<'info> {
    #[account(
        mut,
        seeds = [
            b"user_profile",
            user.key().as_ref()
        ],
        bump = user_profile.bump,
        has_one = user
    )]
    pub user_profile: Account<'info, UserProfile>,

    pub user: Signer<'info>,
}
