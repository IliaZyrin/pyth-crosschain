#[cfg(not(feature = "solana-program"))]
use borsh::{BorshDeserialize, BorshSerialize};
use {
    borsh::BorshSchema,
    serde::{Deserialize, Serialize},
};

/// Id of a feed producing the message. One feed produces one or more messages.
pub type FeedId = [u8; 32];

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, BorshSchema)]
#[cfg_attr(
    feature = "solana-program",
    derive(anchor_lang::AnchorSerialize, anchor_lang::AnchorDeserialize)
)]
#[cfg_attr(
    not(feature = "solana-program"),
    derive(BorshSerialize, BorshDeserialize)
)]
pub struct PriceFeedMessage {
    /// `FeedId` but avoid the type alias because of compatibility issues with Anchor's `idl-build` feature.
    pub feed_id: [u8; 32],
    pub price: i64,
    pub conf: u64,
    pub exponent: i32,
    /// The timestamp of this price update in seconds.
    pub publish_time: i64,
    /// The timestamp of the previous price update.
    pub prev_publish_time: i64,
    pub ema_price: i64,
    pub ema_conf: u64,
}
