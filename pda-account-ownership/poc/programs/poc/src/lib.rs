use anchor_lang::{prelude::*, solana_program::pubkey::PUBKEY_BYTES};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod poc {
    use super::*;
    pub fn create_proxy_escrow(ctx: Context<CreateProxyEscrow>) -> Result<()> {
        let proxy_escrow = &mut ctx.accounts.proxy_escrow;
        msg!(
            "proxy_escrow_owner: {}",
            proxy_escrow.to_account_info().owner
        );
        proxy_escrow.escrow_owner = ctx.accounts.payer.key();
        proxy_escrow.rewards_last_claimed_at = 0;
        Ok(())
    }

    pub fn vulnerable_instruction(ctx: Context<VulnerableInstruction>) -> Result<()> {
        let proxy_escrow = &mut ctx.accounts.proxy_escrow;
        msg!(
            "proxy_escrow_owner: {}",
            proxy_escrow.to_account_info().owner
        );
        proxy_escrow.escrow_owner = ctx.accounts.payer.key();
        proxy_escrow.rewards_last_claimed_at = 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct VulnerableInstruction<'info> {
    /// [Escrow].
    pub proxy_escrow: Account<'info, ProxyEscrow>,

    /// Payer of the initialization.
    #[account(mut)]
    pub payer: Signer<'info>,

    /// System program.
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateProxyEscrow<'info> {
    /// [Escrow].
    #[account(
        init,
        seeds = [
            b"ProxyEscrow".as_ref(),
            payer.key().to_bytes().as_ref()
        ],
        bump,
        payer = payer,
        space = 8 + ProxyEscrow::LEN
    )]
    pub proxy_escrow: Account<'info, ProxyEscrow>,

    /// Payer of the initialization.
    #[account(mut)]
    pub payer: Signer<'info>,

    /// System program.
    pub system_program: Program<'info, System>,
}

/// records rewards state on a user.
#[account]
#[derive(Copy, Debug, Default)]
pub struct ProxyEscrow {
    /// The key of the account that is authorized to withdraw from the [Treasury].
    pub escrow_owner: Pubkey,
    /// When the [proxy_escrow::escrow_owner] last claimed rewards from [Treasury].
    pub rewards_last_claimed_at: i64,
}

impl ProxyEscrow {
    /// Number of bytes in a [ProxyEscrow].
    pub const LEN: usize = PUBKEY_BYTES + 8;
}
