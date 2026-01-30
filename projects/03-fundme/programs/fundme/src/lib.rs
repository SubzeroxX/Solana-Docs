use anchor_lang::prelude::*;

declare_id!("HwKogdG5qRUGXtCZZ65bLG8uGHjBvoXURGj31c8vhHXS");

#[program]
pub mod fundme {
    use anchor_lang::system_program::Transfer;

    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()>{
        let fund_data = &mut ctx.accounts.fund_data;
        fund_data.admin = *ctx.accounts.admin.key;
        fund_data.donations = Vec::new();
        fund_data.bump = ctx.bumps.fund_data;
        msg!("Fund initialized!");
        Ok(())
    }

    pub fn fund(ctx: Context<Fund>, amount: u64) -> Result<()>{
        require!(amount > 1_000_000_000, FundError::TooFewFund);
        require!(amount < 2_000_000_000, FundError::TooMuchFund);
        require!(ctx.accounts.fund_data.donations.len() < 50, FundError::MaxFunder);
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer{
                from: ctx.accounts.user.to_account_info(),
                to: ctx.accounts.fund_data.to_account_info(),
            },
        );
        anchor_lang::system_program::transfer(cpi_context, amount)?;
        let fund_data = &mut ctx.accounts.fund_data;
        let record = Donation {
            donor: *ctx.accounts.user.key,
            amount,
        };
        fund_data.donations.push(record);
        msg!("Donation of {} lamport received!", amount);
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let fund_account = &ctx.accounts.fund_account;
        require_keys_eq!(fund_account.admin, *ctx.accounts.admin.key, FundError::Unauthorized);

        let rent_balance = Rent::get()?.minimum_balance(fund_account.to_account_info().data_len());
        if **fund_account.to_account_info().lamports.borrow() - amount < rent_balance{
            return err!(FundError::InsufficientFunds);
        }

        **fund_account.to_account_info().try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.admin.to_account_info().try_borrow_mut_lamports()? += amount;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init, 
        payer = admin, 
        // 8 (disc) + 32 (admin) + 4 (vec prefix) + (50 * 40 bytes) + 1 (bump)
        space = 8 + 32 + 4 + (50 * 40) + 1, 
        seeds = [b"fund_data"], 
        bump
    )]
    pub fund_data: Account<'info, FundData>,
    
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Fund<'info> {
    #[account(
        mut, 
        seeds = [b"fund_data"], 
        bump // Use the stored bump for speed
    )]
    pub fund_data: Account<'info, FundData>, // Use the same name as above
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub fund_account: Account<'info, FundData>,
    #[account(mut)]
    pub admin: Signer<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub struct Donation {
    pub donor: Pubkey, // 32 bytes
    pub amount: u64,   // 8 bytes
}

#[account]
pub struct FundData {
    pub admin: Pubkey, // 32 bytes
    pub donations: Vec<Donation>,
    pub bump: u8,
}

#[error_code]
pub enum FundError {
    #[msg("You are not authorized to withdraw.")]
    Unauthorized,
    #[msg("Cannot withdraw below rent-exempt minimum.")]
    InsufficientFunds,
    #[msg("Too few amount to fund!")]
    TooFewFund,
    #[msg("Too much amount to fund!")]
    TooMuchFund,
    #[msg("Max funder reached!")]
    MaxFunder
}