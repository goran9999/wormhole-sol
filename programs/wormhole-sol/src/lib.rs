use anchor_lang::prelude::*;
declare_id!("8QS89n567dJk829iueJdxFUeqFkVjRBPU3w674BGmB8m");
pub mod instructions;
pub mod state;
use instructions::*;
#[program]
pub mod wormhole_sol {

    use super::*;

    pub fn emit_wormhole_message(
        ctx: Context<EmitWormholeMessage>,
        payload: Vec<u8>,
        nonce: u32,
    ) -> Result<()> {
        instructions::emit_wormhole_message(ctx, payload, nonce)
    }
}
