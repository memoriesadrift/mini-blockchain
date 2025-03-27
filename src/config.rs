use crate::models::utils::U256;
use ruint::uint;

/// Number of leading zeroes required in block hash
pub const BLOCK_DIFFICULTY: usize = 1;
pub const COIN_DECIMALS: usize = 6;
pub const COINBASE_REWARD: U256 = uint!(1000000_U256);
