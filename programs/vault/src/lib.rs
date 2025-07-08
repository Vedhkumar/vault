#![allow(deprecated)]
// #![allow()]
use std::collections::btree_map::Values;

use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

declare_id!("CuupeR8fLdw1QKfwWDse2fjR166Kj3ZvAyxD1B8EPEb2");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        ctx.accounts.initialize(&ctx.bumps)
    }
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        ctx.accounts.deposit(amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)?;
        Ok(())
    }
    pub fn close(ctx: Context<Close>) -> Result<()> {
        ctx.accounts.clsoe()?;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump,
    )]
    pub vault: SystemAccount<'info>,

    #[account(
        mut,
       seeds = [b"vault", owner.key().as_ref()],
       bump = vault_state.vault_bump,
       close = owner
    )]
    pub vault_state: Account<'info, VaultState>,
    pub system_program: Program<'info, System>,
}

impl<'info> Close<'info> {
    fn close(&self) -> Result<()> {
        let vault_state = self.vault_state.key();
        let signer_seeds: &[&[&[u8]]] = &[&[
            b"vault",
            vault_state.as_ref(),
            &[self.vault_state.vault_bump],
        ]];
        let accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.owner.to_account_info(),
        };
        let ctx = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            accounts,
            signer_seeds,
        );
        transfer(ctx, self.vault.lamports());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump
    )]
    pub vault: SystemAccount<'info>,

    #[account(
       seeds = [b"vault", owner.key().as_ref()],
       bump = vault_state.vault_bump
    )]
    pub vault_state: Account<'info, VaultState>,
    pub system_program: Program<'info, System>,
}

impl<'info> Withdraw<'info> {
    fn withdraw(&self, amount: u64) -> Result<()> {
        let vault_state = self.vault_state.key();
        let signer_seeds: &[&[&[u8]]] = &[&[
            b"vault",
            vault_state.as_ref(),
            &[self.vault_state.vault_bump],
        ]];
        let accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.owner.to_account_info(),
        };
        let ctx = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            accounts,
            signer_seeds,
        );
        transfer(ctx, amount)
    }
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump
    )]
    pub vault: SystemAccount<'info>,

    #[account(
       seeds = [b"vault", owner.key().as_ref()],
       bump = vault_state.vault_bump
    )]
    pub vault_state: Account<'info, VaultState>,
    pub system_program: Program<'info, System>,
}

impl<'info> Deposit<'info> {
    fn deposit(&self, amount: u64) -> Result<()> {
        // CPI
        let cpi_account = Transfer {
            from: self.owner.to_account_info(),
            to: self.vault.to_account_info(),
        };
        let cpi = CpiContext::new(self.system_program.to_account_info(), cpi_account);
        transfer(cpi, amount)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,

    #[account(
        init,
        payer = owner,
        space = 8 + VaultState::INIT_SPACE,
        seeds = [b"state", owner.key().as_ref()],
        bump
    )]
    pub vault_state: Account<'info, VaultState>,

    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
        //? Get the how much rent the vault needed to initialeze the vault because as it is system account it cannot be initialzed in the constraints
        let rent_exempt =
            Rent::get()?.minimum_balance(self.vault_state.to_account_info().data_len());

        transfer(
            CpiContext::new(
                self.system_program.to_account_info(),
                Transfer {
                    from: self.owner.to_account_info(),
                    to: self.vault.to_account_info(),
                },
            ),
            rent_exempt,
        )?;
        self.vault_state.state_bump = bumps.vault_state;
        self.vault_state.vault_bump = bumps.vault;
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct VaultState {
    pub vault_bump: u8,
    pub state_bump: u8,
}
