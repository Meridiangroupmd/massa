use serde::Deserialize;
use time::UTime;

pub const CHANNEL_SIZE: usize = 16;

/// Protocol Configuration
#[derive(Debug, Deserialize, Clone)]
pub struct ProtocolConfig {
    pub ask_block_timeout: UTime,
    pub max_node_known_blocks_size: usize,
    pub max_node_wanted_blocks_size: usize,
}
