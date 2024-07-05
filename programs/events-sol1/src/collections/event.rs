use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Event {
    // datos básicos
    pub name: u32,       // 32 bytes
    pub ticket: u8, // 8 bytes
    pub is_active: bool, // 1 bytes
    pub bump: u8, // 8 bytes
    // cuentas
    pub accepted_mint: Pubkey, // 32 bytes
    pub authority: Pubkey, // 32 bytes
}

impl Event {
    // pub fn increment(&mut self) {
    //     self.count += 1;
    // }

    pub const EVENT_SEED: &'static str = "event";
}