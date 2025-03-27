use crate::{config::BLOCK_DIFFICULTY, models::{block::Block, utils::Hash}};

pub fn mine_block(block: &mut Block) -> &mut Block {
    while !is_valid_blockhash(&block.hash()) {
        block.nonce += 1;
    }

    block
}

fn is_valid_blockhash(hash: &Hash) -> bool {
    hash.starts_with(&[0;BLOCK_DIFFICULTY])
}
