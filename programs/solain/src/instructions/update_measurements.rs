use crate::{states::*};
use anchor_lang::prelude::*;

pub fn update_measurements(
    ctx: Context<UpdateMeasurements>,
    weight_kg: Option<u16>,
    chest_cm: Option<u16>,
    waist_cm: Option<u16>,
    hips_cm: Option<u16>,
    thigh_cm: Option<u16>,
    neck_cm: Option<u16>,
    bicep_cm: Option<u16>,
) -> Result<()> {
    let profile = &mut ctx.accounts.user_profile;
    
    if let Some(weight) = weight_kg {
        profile.weight_kg = weight;
    }
    if let Some(chest) = chest_cm {
        profile.chest_cm = chest;
    }
    if let Some(waist) = waist_cm {
        profile.waist_cm = waist;
    }
    if let Some(hips) = hips_cm {
        profile.hips_cm = hips;
    }
    if let Some(thigh) = thigh_cm {
        profile.thigh_cm = thigh;
    }
    if let Some(neck) = neck_cm {
        profile.neck_cm = neck;
    }
    if let Some(bicep) = bicep_cm {
        profile.bicep_cm = bicep;
    }
    
    profile.last_updated = Clock::get()?.unix_timestamp;
    
    msg!("Measurements updated for: {:?}", profile.user);
    Ok(())
}

#[derive(Accounts)]
pub struct UpdateMeasurements<'info> {
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
