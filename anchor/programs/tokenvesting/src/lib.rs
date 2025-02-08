#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
use anchor_spl::{
  associated_token::AssociatedToken,  
  token_interface::{
    Mint, 
    TokenAccount, 
    TokenInterface,
    TransferChecked,
    transfer_checked
  }
};
//use std::cmp::min;

declare_id!("AgmcNETX3oFtUojT9inACJZaesMErWBfmnDYuqn2Afg");

#[program]
pub mod tokenvesting {

    use super::*;

    pub fn create_vesting_account(
      ctx: Context<CreateVestingAccount>,
      company_name: String
    ) -> Result<()> {

      let vesting_account = &mut ctx.accounts.vesting_account;

        vesting_account.owner = ctx.accounts.signer.key();
        vesting_account.mint = ctx.accounts.mint.key();
        vesting_account.treasury_token_account = ctx.accounts.treasury_token_account.key();
        vesting_account.company_name = company_name;
        vesting_account.treasury_bump = ctx.bumps.treasury_token_account;
        vesting_account.bump = ctx.bumps.vesting_account;

      Ok(())
    } 

    pub fn create_employee_account(
      ctx: Context<CreateEmployeeAccount>,
      start_time: i64,
      end_time: i64,
      total_amount: u64,
      cliff_time: i64
    ) -> Result<()> {

      let employee_account = &mut ctx.accounts.employee_account;
        
        employee_account.beneficiary = ctx.accounts.beneficiary.key();
        employee_account.start_time = start_time;
        employee_account.end_time = end_time;
        employee_account.cliff_time = cliff_time;
        employee_account.vesting_account = ctx.accounts.vesting_account.key();
        employee_account.total_amount = total_amount;
        employee_account.total_withdrawn = 0;
        employee_account.bump = ctx.bumps.employee_account;

      Ok(())
    }

    pub fn claim_tokens(
      ctx: Context<ClaimTokens>,
      _company_name: String
    ) -> Result<()> {

      let employee_account = &mut ctx.accounts.employee_account;

      let now = Clock::get()?.unix_timestamp;

      if now < employee_account.cliff_time {
        return Err(ErrorCode::ClaimNotAvailableYet.into())
      }

      let time_since_start = now.saturating_sub(employee_account.start_time);
      let total_vesting_time = employee_account.end_time.saturating_sub(employee_account.start_time);

      if total_vesting_time == 0 {
        return Err(ErrorCode::InvalidVestingPeriod.into())
      }

      let vested_amount = if now >= employee_account.end_time {
        employee_account.total_amount
      } else {
          match employee_account.total_amount.checked_mul(time_since_start as u64) {
            Some(product) => 
              product / total_vesting_time as u64
            ,
            None => {
              return Err(ErrorCode::CalculationOverflow.into())
            }
          }
      };

      let claimable_amount = vested_amount.saturating_sub(employee_account.total_withdrawn);

      if claimable_amount == 0 {
        return Err(ErrorCode::NothingToClaim.into())
      }

      let transfer_cpi_accounts = TransferChecked{ 
        from: ctx.accounts.treasury_token_account.to_account_info(), 
        mint: ctx.accounts.mint.to_account_info(), 
        to: ctx.accounts.employee_token_account.to_account_info(), 
        authority: ctx.accounts.treasury_token_account.to_account_info() 
      };

      let cpi_program = ctx.accounts.token_program.to_account_info();

      let signer_seeds: &[&[&[u8]]] = &[
        &[ 
          // SEEDS
          b"vesting_treasury",
          ctx.accounts.vesting_account.company_name.as_ref(),
          // BUMP
          &[ctx.accounts.vesting_account.treasury_bump]
        ]
      ];

      let cpi_context = CpiContext::new_with_signer(
        cpi_program, 
        transfer_cpi_accounts, 
        signer_seeds
      );

      let decimals = ctx.accounts.mint.decimals;

      transfer_checked(
        cpi_context, 
        claimable_amount as u64, 
        decimals
      )?;

      employee_account.total_withdrawn += claimable_amount;

      Ok(())
    }



  
}

#[derive(Accounts)]
#[instruction(company_name: String)]
pub struct CreateVestingAccount<'info> {

  // The signer of the instruction
  #[account(mut)]
  pub signer: Signer<'info>,

  #[account(
    init,
    space = 8 + VestingAccount::INIT_SPACE,
    payer = signer,
    seeds = [company_name.as_ref()],
    bump
  )]
  // Account that holds all of the state for the 
  // new vesting account.
  pub vesting_account: Account<'info, VestingAccount>,

  // Mint of the SPL token we are vesting
  pub mint: InterfaceAccount<'info, Mint>,

  #[account(
    init,
    token::mint = mint,
    token::authority = treasury_token_account,
    payer = signer,
    seeds = [
      b"vesting_treasury",
      company_name.as_bytes(),
      //&company_name.as_bytes()[..min(company_name.len(), 16)] // to make the seed &[u8; 16]
    ],
    bump
  )]
  // Treasury Token Account that holds the SPL tokens
  // we are going to give to the employees
  pub treasury_token_account: InterfaceAccount<'info, TokenAccount>,


  pub system_program: Program<'info, System>,
  pub token_program: Interface<'info, TokenInterface>
}

#[derive(Accounts)]
pub struct CreateEmployeeAccount<'info> {

  // The employeer
  #[account(mut)]
  pub owner: Signer<'info>,

  // The employee's pubkey
  pub beneficiary: SystemAccount<'info>,

  // The employee's vesting account created by the employeer
  #[account(
    has_one = owner
  )]
  pub vesting_account: Account<'info, VestingAccount>,

  // The employee's state account
  #[account(
    init,
    space = 8 + EmployeeAccount::INIT_SPACE,
    payer = owner,
    seeds = [
      b"employee_vesting",
      beneficiary.key().as_ref(),
      vesting_account.key().as_ref()
    ],
    bump
  )]
  pub employee_account: Account<'info, EmployeeAccount>,

  pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(company_name: String)]
pub struct ClaimTokens<'info> {

  #[account(mut)]
  pub beneficiary: Signer<'info>,

  #[account(
    mut,
    seeds = [
      b"employee_vesting",
      beneficiary.key().as_ref(),
      vesting_account.key().as_ref()
    ],
    bump = employee_account.bump,
    has_one = beneficiary,
    has_one = vesting_account,
  )]
  pub employee_account: Account<'info, EmployeeAccount>,

  #[account(
    mut,
    seeds = [company_name.as_ref()],
    bump = vesting_account.bump,
    has_one = treasury_token_account,
    has_one = mint
  )]
  pub vesting_account: Account<'info, VestingAccount>,

  pub mint: InterfaceAccount<'info, Mint>,

  pub treasury_token_account: InterfaceAccount<'info, TokenAccount>,

  #[account(
    init_if_needed,
    payer = beneficiary,
    associated_token::mint = mint,
    associated_token::authority = beneficiary,
    associated_token::token_program = token_program,
  )]
  pub employee_token_account: InterfaceAccount<'info, TokenAccount>,

  pub system_program: Program<'info, System>,
  pub token_program: Interface<'info, TokenInterface>,
  pub associated_token_program: Program<'info, AssociatedToken>


} 

#[account]
#[derive(InitSpace)]
pub struct VestingAccount {
    pub owner: Pubkey,                    // Whoever has permissions over this vesting account
    pub mint: Pubkey,                     // Mint of the SPL token in the vesting account 
    pub treasury_token_account: Pubkey,   // Token Account of the employeer that stores the spl tokens
    #[max_len(50)]
    pub company_name: String,             // Company name (for a seed)
    pub treasury_bump: u8,                // Bump of the Token Account
    pub bump: u8                          // Bump of the vesting account
}

#[account]
#[derive(InitSpace)]
pub struct EmployeeAccount {
  pub beneficiary: Pubkey,      // Public key of the employee receiving the tokens.
  pub start_time: i64,          // Vesting start time
  pub end_time: i64,            // Vesting end time
  pub cliff_time: i64,          // Time after which employee can access tokens.
  pub vesting_account: Pubkey,  // Public key of the related vesting account.
  pub total_amount: u64,        // Total amount of tokens assigned to the employee.
  pub total_withdrawn: u64,     // Total amount of tokens the employee has withdrawn.
  pub bump: u8                  // Bump   
}

#[error_code]
pub enum ErrorCode {
  #[msg("Claim not available yet")]
  ClaimNotAvailableYet,

  #[msg("Invalid vesting period")]
  InvalidVestingPeriod,

  #[msg("Calculation Overflow")]
  CalculationOverflow,

  #[msg("Nothing to claim")]
  NothingToClaim

}