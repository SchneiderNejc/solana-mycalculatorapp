use anchor_lang::prelude::*;

declare_id!("39YeeYEbuTkW9V39ynusS7FQNT59aGKEx1RZSAMg7JNr");

#[program]
pub mod mycalculatorapp {
    use super::*;

    pub fn create(ctx: Context<Create>, init_message: String) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.greeting = init_message;
        ok(())
    }
}

#derrive(Accounts)
pub struct create<'info> {
    #[account(init, payer=user, space=264)]
    pub calculator: Account<'info, Calculator>
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[account]
pub struct Calculator {
}