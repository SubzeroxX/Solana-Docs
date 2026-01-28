use anchor_lang::prelude::*;

declare_id!("BY8LCLmCXe8chKjiP4F8Qeryz7aZ72e6vPz8C7u4HSuh");

#[program]
pub mod hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
