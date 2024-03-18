use anchor_lang::prelude::*;

declare_id!("39YeeYEbuTkW9V39ynusS7FQNT59aGKEx1RZSAMg7JNr");

#[program]
pub mod mycalculatorapp {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
