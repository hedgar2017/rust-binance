//!
//! The order time in force.
//!

use serde::Deserialize;

///
/// The order time-in-force. See the below descriptions.
///
#[derive(Debug, Deserialize, Clone, Copy)]
pub enum TimeInForce {
    /// The default time-in-force. Effective until the trade is executed or cancelled.
    #[serde(rename = "GTC")]
    GoodTilCanceled,
    /// Must be filled immediately or is cancelled.
    #[serde(rename = "IOC")]
    ImmediateOrCancel,
    /// Is cancelled if the entire order does not execute as soon as it becomes available.
    #[serde(rename = "FOK")]
    FillOrKill,
}

impl ToString for TimeInForce {
    fn to_string(&self) -> String {
        match self {
            TimeInForce::GoodTilCanceled => "GTC",
            TimeInForce::ImmediateOrCancel => "IOC",
            TimeInForce::FillOrKill => "FOK",
        }
        .to_owned()
    }
}
