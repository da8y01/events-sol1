use {
    anchor_lang::prelude::*,
    crate::instructions::*
};

mod collections;
mod instructions;

declare_id!("9vhSousnsM3zDPwhL4QUef5tsrRZBaQjdopaw8WZmDp4");

#[program]
pub mod events_sol1 {
    use super::*;

    pub fn create(_ctx: Context<Create>) -> Result<()> {
        Ok(())
    }
}
