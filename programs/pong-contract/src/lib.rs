use anchor_lang::prelude::*;
use spl_token::solana_program::native_token::sol_to_lamports;

declare_id!("9UrX38GgCX7EuYvzye6GzkfN2X38DiM7xF2isWGbpzdQ");

static SEED: &[u8] = b"authorityy";

#[program]
pub mod pong_contract {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, _authority_bump: u8) -> ProgramResult {
        ctx.accounts.escrow.trusted_server = ctx.accounts.trustedserver.key();
        Ok(())
    }

    pub fn paypiper(ctx: Context<PayPiper>, _authority_bump: u8) -> ProgramResult {
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.newplayer.key(),
            &ctx.accounts.escrow.key(),
            sol_to_lamports(1.0),
        );

        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.newplayer.to_account_info(),
                ctx.accounts.escrow.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        ctx.accounts
            .escrow
            .un_gamed_players
            .push(ctx.accounts.newplayer.key());

        Ok(())
    }

    pub fn cancel(ctx: Context<Cancel>, _authority_bump: u8) -> ProgramResult {
        if ctx
            .accounts
            .escrow
            .un_gamed_players
            .contains(&ctx.accounts.newplayer.key())
            && ctx.accounts.escrow.trusted_server == ctx.accounts.trustedserver.key()
        {
            **ctx
                .accounts
                .escrow
                .to_account_info()
                .try_borrow_mut_lamports()? -= sol_to_lamports(0.1);

            **ctx
                .accounts
                .newplayer
                .to_account_info()
                .try_borrow_mut_lamports()? += sol_to_lamports(0.1);
            let us_key = ctx.accounts.newplayer.key();
            ctx.accounts
                .escrow
                .un_gamed_players
                .retain(|&player| player != us_key);
        }
        Ok(())
    }

    pub fn matchplayers(ctx: Context<MatchPlayers>, _authority_bump: u8) -> ProgramResult {
        let pone_key = ctx.accounts.playerone.key();
        let ptwo_key = ctx.accounts.playertwo.key();
        if ctx.accounts.escrow.un_gamed_players.contains(&pone_key)
            && ctx.accounts.escrow.un_gamed_players.contains(&ptwo_key)
        {
            ctx.accounts
                .escrow
                .un_gamed_players
                .retain(|&player| player != pone_key);
            ctx.accounts
                .escrow
                .un_gamed_players
                .retain(|&player| player != ptwo_key);
            ctx.accounts.newescrow.player_one = pone_key;
            ctx.accounts.newescrow.player_two = ptwo_key;

            **ctx
                .accounts
                .escrow
                .to_account_info()
                .try_borrow_mut_lamports()? -= sol_to_lamports(0.18);

            **ctx
                .accounts
                .newescrow
                .to_account_info()
                .try_borrow_mut_lamports()? += sol_to_lamports(0.18);

            **ctx
                .accounts
                .escrow
                .to_account_info()
                .try_borrow_mut_lamports()? -= sol_to_lamports(0.02);
            **ctx
                .accounts
                .trustedserver
                .to_account_info()
                .try_borrow_mut_lamports()? += sol_to_lamports(0.02);
        }
        Ok(())
    }

    pub fn declarewinner(ctx: Context<DeclareWinner>, _authority_bump: u8) -> ProgramResult {
        if ctx.accounts.escrow.trusted_server == ctx.accounts.trustedserver.key() {
            **ctx
                .accounts
                .newescrow
                .to_account_info()
                .try_borrow_mut_lamports()? -= sol_to_lamports(0.18);

            **ctx
                .accounts
                .winner
                .to_account_info()
                .try_borrow_mut_lamports()? += sol_to_lamports(0.18);

            let remaining_lamports = **ctx
                .accounts
                .newescrow
                .to_account_info()
                .try_borrow_mut_lamports()?;

            **ctx
                .accounts
                .newescrow
                .to_account_info()
                .try_borrow_mut_lamports()? -= remaining_lamports;

            **ctx
                .accounts
                .trustedserver
                .to_account_info()
                .try_borrow_mut_lamports()? += remaining_lamports;
        }
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(authority_bump: u8)]
pub struct Initialize<'info> {
    trustedserver: Signer<'info>,
    #[account(init, seeds = [SEED], bump = authority_bump, payer = trustedserver, space=300)]
    escrow: Account<'info, Escrow>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(authority_bump: u8)]
pub struct PayPiper<'info> {
    trustedserver: Signer<'info>,
    #[account(mut)]
    newplayer: Signer<'info>,
    #[account(mut)]
    escrow: Account<'info, Escrow>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(authority_bump: u8)]
pub struct Cancel<'info> {
    trustedserver: Signer<'info>,
    #[account(mut)]
    newplayer: Signer<'info>,
    #[account(mut)]
    escrow: Account<'info, Escrow>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(authority_bump: u8)]
pub struct MatchPlayers<'info> {
    trustedserver: Signer<'info>,
    #[account(mut)]
    playerone: AccountInfo<'info>,
    #[account(mut)]
    playertwo: AccountInfo<'info>,
    #[account(mut)]
    escrow: Account<'info, Escrow>,
    #[account(init, seeds = [playerone.key().as_ref(), playertwo.key().as_ref()], bump = authority_bump, payer = trustedserver, space=80)]
    newescrow: Account<'info, NewEscrow>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(authority_bump: u8)]
pub struct DeclareWinner<'info> {
    trustedserver: Signer<'info>,
    #[account(mut)]
    winner: AccountInfo<'info>,
    #[account(mut)]
    newescrow: Account<'info, NewEscrow>,
    #[account(mut)]
    escrow: Account<'info, Escrow>,
    system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct Escrow {
    un_gamed_players: Vec<Pubkey>,
    trusted_server: Pubkey,
}

#[account]
#[derive(Default)]
pub struct NewEscrow {
    player_one: Pubkey,
    player_two: Pubkey,
}
