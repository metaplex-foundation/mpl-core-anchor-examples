pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("GD7rkX6ARmc4SZSVTCTURggAHXAHVEyeTG3bNtcUZZAE");

#[program]
pub mod mpl_core_anchor_examples {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}
