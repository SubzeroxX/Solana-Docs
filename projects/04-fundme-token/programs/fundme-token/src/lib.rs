use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint};
use anchor_spl::associated_token::AssociatedToken;

declare_id!("8UpsxRe1db8Xy3BbFacAuWsYT5xUnk1dGyrfXc4jT3Mr");

#[program]
pub mod fundme_token {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let fund_data = &mut ctx.accounts.fund_data;
        fund_data.admin = *ctx.accounts.admin.key;
        fund_data.donations = Vec::new();
        fund_data.bump = ctx.bumps.fund_data;
        Ok(())
    }

    pub fn fund(ctx: Context<Fund>, amount: u64) -> Result<()> {
        require!(amount >= 1_000_000, FundError::TooFewFund);
        
        let donor_pubkey = *ctx.accounts.user.key;
        let fund_data = &mut ctx.accounts.fund_data;

        let donor_index = fund_data.donations.iter().position(|d| d.donor == donor_pubkey);

        if let Some(index) = donor_index {
            fund_data.donations[index].amount += amount;
            msg!("Returning donor. New donation: {} unit", amount);
        } else {
            let new_size = fund_data.to_account_info().data_len() + 40;

            secure_realloc(
                &fund_data.to_account_info(),
                &ctx.accounts.user.to_account_info(),
                &ctx.accounts.system_program.to_account_info(),
                new_size,
            )?;

            fund_data.donations.push(Donation {
                donor: donor_pubkey,
                amount,
            });
            msg!("New donor and storage reserved!");
        }

        let cpi_accounts = token::Transfer {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.vault_token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        token::transfer(cpi_ctx, amount)?;

        msg!("New USDC donation: {}", amount);
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let fund_data = &ctx.accounts.fund_data;
        
        require_keys_eq!(fund_data.admin, *ctx.accounts.admin.key, FundError::Unauthorized);
        
        let bump = fund_data.bump; 
        let seeds = &[
            b"fund_data".as_ref(),
            &[bump],
        ];
        let signer = &[&seeds[..]];
        
        let cpi_accounts = token::Transfer {
            from: ctx.accounts.vault_token_account.to_account_info(),
            to: ctx.accounts.admin_token_account.to_account_info(),
            authority: fund_data.to_account_info(), 
        };
        
        let cpi_program = ctx.accounts.token_program.to_account_info();
        
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
    
        token::transfer(cpi_ctx, amount)?;
    
        msg!("Withdrawal of {} tokens successful!", amount);
        Ok(())
    }

}


pub fn secure_realloc<'info>(
    account_to_realloc: &AccountInfo<'info>,
    payer: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    new_size: usize,
) -> Result<()> {
    let current_len = account_to_realloc.data_len();
    
    if new_size > current_len {
        let rent = Rent::get()?;
        let new_minimum_balance = rent.minimum_balance(new_size);
        let lamports_diff = new_minimum_balance.saturating_sub(account_to_realloc.lamports());

        if lamports_diff > 0 {
            anchor_lang::system_program::transfer(
                CpiContext::new(
                    system_program.clone(),
                    anchor_lang::system_program::Transfer {
                        from: payer.clone(),
                        to: account_to_realloc.clone(),
                    },
                ),
                lamports_diff,
            )?;
        }

        account_to_realloc.resize(new_size)?;
    }

    Ok(())
}


#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init, 
        payer = admin, 
        space = 8 + 32 + 4 + 1, 
        seeds = [b"fund_data"], 
        bump
    )]
    pub fund_data: Account<'info, FundData>,

    // A token, amit el akarunk fogadni (pl. USDC mint címe)
    pub token_mint: Account<'info, Mint>,

    // A program saját token számlája (Vault), amit a fund_data PDA birtokol
    #[account(
        init,
        payer = admin,
        associated_token::mint = token_mint,
        associated_token::authority = fund_data,
    )]
    pub vault_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Fund<'info> {
    #[account(mut, seeds = [b"fund_data"], bump = fund_data.bump)]
    pub fund_data: Account<'info, FundData>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub vault_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct FundData {
    pub admin: Pubkey,
    pub donations: Vec<Donation>,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub struct Donation {
    pub donor: Pubkey,
    pub amount: u64,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut, seeds = [b"fund_data"], bump = fund_data.bump)]
    pub fund_data: Account<'info, FundData>,

    #[account(mut)]
    pub vault_token_account: Account<'info, TokenAccount>,

    // Az admin saját token számlája (ide érkezik a pénz)
    #[account(mut)]
    pub admin_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub admin: Signer<'info>,
    pub token_program: Program<'info, Token>,
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