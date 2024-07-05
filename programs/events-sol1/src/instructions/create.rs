use {
    anchor_lang::prelude::*,
    crate::collections::event::*
};

#[derive(Accounts)]
pub struct Create<'info> {
    // 8 bytes para discriminador  + (lo que ocupe tu estructura)
    #[account(
        init,
        seeds = [Event::EVENT_SEED.as_ref(), authority.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + Event::INIT_SPACE)
    ]
    pub event: Account<'info, Event>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn create(_ctx: Context<Create>) -> Result<()> {
    _ctx.accounts.event.authority = _ctx.accounts.authority.key();
    _ctx.accounts.event.accepted_mint = _ctx.accounts.authority.key();
    _ctx.accounts.event.is_active = false;
    _ctx.accounts.event.ticket = 0;
    _ctx.accounts.event.name = 0;
    _ctx.accounts.event.bump = _ctx.bumps.event;
    Ok(())
}