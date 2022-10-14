use anchor_lang::prelude::*;

declare_id!("8eRLNRfCsDbAgXuFSHufEjeM1VJEYZGCtSoS9WZmfKZD");

#[program]
pub mod arising {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
