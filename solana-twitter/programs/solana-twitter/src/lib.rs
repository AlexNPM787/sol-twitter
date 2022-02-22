use anchor_lang::prelude::*;

declare_id!("7v3oKCjcm4FzfF5rvqePA9YgcbnVKQv2YAHCDKaFCC4T");

#[program]
pub mod solana_twitter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
