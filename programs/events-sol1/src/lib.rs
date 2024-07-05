use anchor_lang::prelude::*;

declare_id!("9vhSousnsM3zDPwhL4QUef5tsrRZBaQjdopaw8WZmDp4");

#[program]
pub mod events_sol1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
