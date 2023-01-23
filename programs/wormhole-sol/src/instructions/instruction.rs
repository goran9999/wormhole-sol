use anchor_lang::{prelude::Pubkey, InstructionData};
use solana_program::instruction::Instruction;

use wormhole_sdk::{config, fee_collector, id, sequence};

pub fn send_message_ix(
    program_id: Pubkey,
    emitter: Pubkey,
    message: Pubkey,
    payer: Pubkey,
    payload: Vec<u8>,
) {
    let wormhole = id();
    let config = config(&wormhole);
    let fee_collector = fee_collector(&wormhole);
    let sequence = sequence(&wormhole, &emitter);
}
