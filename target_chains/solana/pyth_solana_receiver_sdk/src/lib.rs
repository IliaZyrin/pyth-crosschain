// We can't do much about the size of `anchor_lang::error::Error`.
#![allow(clippy::result_large_err)]

use anchor_lang::{declare_id, prelude::Pubkey, pubkey};

pub mod error;
pub mod price_update;

cfg_if::cfg_if! {
    if #[cfg(feature = "lazer")] {
        declare_id!("rec2HHDDnjLfj4kE7VyEtFA1HPGQLK33259532cRyHp");
        pub const PYTH_PUSH_ORACLE_ID: Pubkey = pubkey!("pyt2F414BA6dPttK6RddPZUdHfapoBN24GL5wbrPCou");
    } else {
        declare_id!("rec5EKMGg6MxZYaMdyBfgwp4d5rB9T1VQH5pJv5LtFJ");
        pub const PYTH_PUSH_ORACLE_ID: Pubkey = pubkey!("pythWSnswVUd12oZpeFP8e9CVaEqJg25g1Vtc2biRsT");
    }
}
