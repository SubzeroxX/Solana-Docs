# 04-Fundme-token

### Specification

Our next program will be a simple fundme.
They will send in USDC token.
We will store the sender address, the sended amount and
anyone can request these data.
The contract owner can withdraw the full amount to his own address.

We will create our new program [Hint](01-basic-commands.md#project-creation)
Install the dependencies [Hint](01-basic-commands.md#installing-dependencies)

In case of coins we will need to add `anchor_spl` via

```sh
cargo add anchor_spl
```

Also modifing the idl-build under `features` category to:

```toml
idl-build = ["anchor-lang/idl-build", "anchor-spl/idl-build"]
```

This will ensure to build `anchol-spl` too for the project

### Programming section

First function initialize this will setup our fund account and admin account

For that we create a new data structure for initialization
~~We maxed the account number to 50~~
We now using a dinamic reallocation if there is a new user.

```rust
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,  // init: create a new account on chain
        payer = admin, // payer: who pays the rent for storage
        space = 8 + 32 + 4 + 1, // 8 (program id) + 32 (admin) + 4 (vec prefix) + 40 bytes (address + sent value) + 1 (bump)
        seeds = [b"fund_data"], // id for finding the data
        bump // anchor for find the new bump
    )]
    pub fund_data: Account<'info, FundData>,

    // Accepted token type address eg. USDC
    pub token_mint: Account<'info, Mint>,

    // A program saját token számlája (Vault), amit a fund_data PDA birtokol
    #[account(
        init, // init: create a new account on chain
        payer = admin, // payer: who pays the rent for storage
        associated_token::mint = token_mint, // setting the token
        associated_token::authority = fund_data, // the owner is fund_data PDA
    )]
    pub vault_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub admin: Signer<'info>, 
    // admin: the initializor will be the admin
    pub system_program: Program<'info, System>, 
    // system_program: rights for creating new account and/or transfer Sol
    pub token_program: Program<'info, Token>,
    // token_program: Official Solana Token Program logic
    pub associated_token_program: Program<'info, AssociatedToken>,
    // associated_token_program: helper program for create Associated Token Accounts (ATAs)
    pub rent: Sysvar<'info, Rent>,
    // rent: provides the current cost of storing data on-chain.
}
```

We create an initialize function where we set up the admin account, 
the logging and the marking for the data list

```rust
pub fn initialize(ctx: Context<Initialize>) -> Result<()>{
    let fund_data = &mut ctx.accounts.fund_data;
    fund_data.admin = *ctx.accounts.admin.key;
    fund_data.donations = Vec::new();
    fund_data.bump = ctx.bumps.fund_data;
    msg!("Fund initialized!");
    Ok(())
}
```

We create a two new data structure for Funding

```rust
#[derive(Accounts)]
pub struct Fund<'info> {
    #[account(mut, seeds = [b"fund_data"], bump = fund_data.bump)]
    pub fund_data: Account<'info, FundData>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    // user_token_account: User's usdc token account

    #[account(mut)]
    pub vault_token_account: Account<'info, TokenAccount>,
    // vault_token_account: programs USDC account

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct FundData {
    pub admin: Pubkey, // 32 bytes
    // admin: admin's address
    pub donations: Vec<Donation>,
    // donations: list of funders and amounts
    pub bump: u8,
    // donation list id
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub struct Donation {
    pub donor: Pubkey,
    // donor: funders address
    pub amount: u64,
    // amount: funded amount
}
```

Then comes the fund function

Firstly we write our resizer for storing the donation

```rust
pub fn secure_realloc<'info>(
    account_to_realloc: &AccountInfo<'info>,
    payer: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    new_size: usize,
) -> Result<()> {
    let current_len = account_to_realloc.data_len();
    
    // only do anything if it has to grow
    if new_size > current_len {
        let rent = Rent::get()?;
        let new_minimum_balance = rent.minimum_balance(new_size);
        let lamports_diff = new_minimum_balance.saturating_sub(account_to_realloc.lamports());

        // getting extra price for rent from the client (only if new donator)
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
        // the resize happens here
        account_to_realloc.resize(new_size)?;
    }

    Ok(())
}
```

```rust
pub fn fund(ctx: Context<Fund>, amount: u64) -> Result<()> {    
    let donor_pubkey = *ctx.accounts.user.key;
    let fund_data = &mut ctx.accounts.fund_data;

    let donor_index = fund_data.donations.iter().position(|d| d.donor == donor_pubkey);
    //finding donors index

    if let Some(index) = donor_index { // if a returning donor
        fund_data.donations[index].amount += amount;
        msg!("Returning donor. New donation: {} unit", amount);
    } else { // if a new donor
        let new_size = fund_data.to_account_info().data_len() + 40;

        // using our own resizer for the data 
        // to not trigger every time only when its needed
        secure_realloc(
             &fund_data.to_account_info(),
             &ctx.accounts.user.to_account_info(),
             &ctx.accounts.system_program.to_account_info(),
             new_size,
         )?;

        // storing the donation data
        fund_data.donations.push(Donation {
            donor: donor_pubkey,
            amount,
        });
        msg!("New donor and storage reserved!");
    }

    // defining the transaction method using CPI
    let cpi_accounts = token::Transfer {
        from: ctx.accounts.user_token_account.to_account_info(),
        to: ctx.accounts.vault_token_account.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };

    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    token::transfer(cpi_ctx, amount)?; // executing the transfer

    msg!("New USDC donation: {}", amount);
    Ok(())
}
```

Firstly we create a new CpiContext
which is Cross-Program Invocation (CPI) Context. That means you can invoke
a program from an another program. This is specified for transfers direction
and the transfer data.
Then we 'call' execute the program and return a message from the success.

Then comes withdraw's data structure

```rust
#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut, seeds = [b"fund_data"], bump = fund_data.bump)]
    pub fund_data: Account<'info, FundData>,

    #[account(mut)]
    pub vault_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub admin_token_account: Account<'info, TokenAccount>,
    // admin_token_account: admin's USDC account

    #[account(mut)]
    pub admin: Signer<'info>,
    pub token_program: Program<'info, Token>,
}
```

Then the withdraw function it self:

```rust
pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    let fund_data = &ctx.accounts.fund_data;
    require_keys_eq!(fund_data.admin, *ctx.accounts.admin.key,FundError::Unauthorized);
    // check if the signer is the admin unless throwing an error
    
    let bump = fund_data.bump; 
    let seeds = &[
        b"fund_data".as_ref(),
        &[bump],
    ];
    let signer = &[&seeds[..]];
    // bumping to the new signature

    let cpi_accounts = token::Transfer {
        from: ctx.accounts.vault_token_account.to_account_info(),
        to: ctx.accounts.admin_token_account.to_account_info(),
        authority: fund_data.to_account_info(), 
    };
    
    let cpi_program = ctx.accounts.token_program.to_account_info();
    
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts,signer);

    token::transfer(cpi_ctx, amount)?;

    msg!("Withdrawal of {} tokens successful!", amount);
    Ok(())
}
```

No, we cannot use the fund's function because
the from account always has to be the signer.

We use basic error messages for reporting errors.

```rs
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
```

Alternative way, using Program Derived Address (PDA).
These accounts are owned by the program logic and using math to sign the smart contract
