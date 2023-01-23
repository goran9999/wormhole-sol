use crate::state::MessageConfig;
use anchor_lang::prelude::{Accounts, *};
use std::mem::size_of;
use wormhole_sdk::*;
#[derive(Accounts)]
pub struct EmitWormholeMessage<'info> {
    #[account(init_if_needed,seeds=[b"message-config",payer.key().as_ref()],bump,payer=payer,space=8+size_of::<MessageConfig>())]
    pub message_conifg: Account<'info, MessageConfig>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut,seeds=[b"message",wormhole_config.key().as_ref(),payer.key().as_ref(),&(message_conifg.index+1).to_le_bytes()],bump)]
    ///CHECK
    pub message: AccountInfo<'info>,
    #[account(mut,seeds=[b"Bridge"],bump,seeds::program=id())]
    ///CHECK:seeds checked
    pub wormhole_config: UncheckedAccount<'info>,
    #[account()]
    ///CHECK
    pub emitter_address: UncheckedAccount<'info>,
    #[account(mut)]
    ///CHECK
    pub fee_collector: UncheckedAccount<'info>,
    #[account(mut)]
    ///CHECK
    pub sequence: UncheckedAccount<'info>,
    ///CHECK
    #[account()]
    pub wormhole: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub clock: Sysvar<'info, Clock>,
}

pub fn emit_wormhole_message(
    ctx: Context<EmitWormholeMessage>,
    payload: Vec<u8>,
    nonce: u32,
) -> Result<()> {
    let post_message_accs: Vec<AccountInfo> = vec![
        ctx.accounts.payer.to_account_info(),
        ctx.accounts.emitter_address.to_account_info(),
        ctx.accounts.message.to_account_info(),
        ctx.accounts.wormhole_config.to_account_info(),
        ctx.accounts.fee_collector.to_account_info(),
        ctx.accounts.sequence.to_account_info(),
        ctx.accounts.wormhole.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
        ctx.accounts.rent.to_account_info(),
        ctx.accounts.clock.to_account_info(),
    ];

    post_message(
        ctx.program_id.clone(),
        ctx.accounts.payer.key(),
        ctx.accounts.message.key(),
        payload,
        ConsistencyLevel::Confirmed,
        Some(&[&[
            "message".as_bytes(),
            ctx.accounts.wormhole_config.key.as_ref(),
            ctx.accounts.payer.key.as_ref(),
            &(ctx.accounts.message_conifg.index + 1).to_le_bytes(),
            &[*ctx.bumps.get(&"message".to_string()).unwrap()],
        ]]),
        &post_message_accs,
        nonce,
    )?;

    ctx.accounts.message_conifg.index = ctx.accounts.message_conifg.index.checked_add(1).unwrap();

    Ok(())
}
