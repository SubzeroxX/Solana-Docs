# 03-Fundme

### Specification

Our next program will be a simple fundme.
They will send in solona.
We will store the sender address, the sended amount and
anyone can request these data.
The contract owner can withdraw the full amount to his own address.

We will create our new program [Hint](01a-programs.md#creating-program)
Adding the dependencies [Hint](01a-programs.md#installing-dependencies)
Adding the basic stuffs [Hint](01a-programs.md#settings-create-type)

### Programming section

First function initialize this will setup our fund account and admin account

For that we create a new data structure for initialization

```rust
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init, // init: create a new account on chain
        payer = admin, // payer: who pays the rent for storage
        // 8 (program id) + 32 (admin) + 4 (vec prefix) + (50 * 40 bytes) + 1 (bump)
        // max 50 donators
        space = 8 + 32 + 4 + (50 * 40) + 1, // pace: memory reservation 
        seeds = [b"fund_data"], 
        bump
    )]
    pub fund_data: Account<'info, FundData>,
    #[account(mut)]
    // mut: marker for Sol balance will change
    pub admin: Signer<'info>,
    // Signer: authentication for transaction (Clicking approve or reject)
    pub system_program: Program<'info, System>,
    // system_program: rights for creating new account and/or transfer Sol
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
    #[account(
        mut, 
        seeds = [b"fund_data"], 
        bump // Use the stored bump for speed
    )]
    pub fund_data: Account<'info, FundData>,
    // fund_account: account to be fund we call the FundData for the account's address
    #[account(mut)]
    pub user: Signer<'info>,
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

```rust
pub fn fund(ctx: Context<Fund>, amount: u64) -> Result<()>{
    // if we on the 50th donator then we will run out of storage
    require!(ctx.accounts.fund_data.donations.len() < 50, FundError::MaxFunder);
    let cpi_context = CpiContext::new(
        // program's account
        ctx.accounts.system_program.to_account_info(),
        // method of moving Sol
        Transfer{
            from: ctx.accounts.user.to_account_info(), // donator
            to: ctx.accounts.fund_data.to_account_info(),// vault
        },
    );
    anchor_lang::system_program::transfer(cpi_context, amount)?;
    // execute the cross-program invocation with the following amount
    let fund_data = &mut ctx.accounts.fund_data;
    // record data
    let record = Donation {
        donor: *ctx.accounts.user.key,
        amount,
    };
    fund_data.donations.push(record);
    msg!("Donation of {} lamport received!", amount);
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
    #[account(mut)]
    pub fund_account: Account<'info, FundData>,
    #[account(mut)]
    pub admin: Signer<'info>,
}
```

Then the withdraw function it self:

```rust
pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    let fund_account = &ctx.accounts.fund_account;
    require_keys_eq!(fund_account.admin, *ctx.accounts.admin.key, FundError::Unauthorized);
    // check if the signer is the admin unless throwing an error
    let rent_balance = Rent::get()?.minimum_balance(fund_account.to_account_info().data_len());
    // request the balance
    if **fund_account.to_account_info().lamports.borrow() - amount < rent_balance{
        // check for the requested amount is not too much
        return err!(FundError::InsufficientFunds);
    }

    // manual moving the money
    **fund_account.to_account_info().try_borrow_mut_lamports()? -= amount;
    // subtracting amount from the vault
    **ctx.accounts.admin.to_account_info().try_borrow_mut_lamports()? += amount;
    // adding amount to the admin account

    Ok(())
}
```

No, we cannot use the fund's function because
the from account always has to be the signer.

Alternative way, using Program Derived Address (PDA).
These accounts are owned by the program logic and using math to sign the smart contract

The contract's account must stay above 1Sol for paying the rent for storage

*For testing limitation max 20Sol is fundable*
