//!
//! The order POST response result.
//!

use rust_decimal::Decimal;
use serde::Deserialize;

use crate::data::order_side::OrderSide;
use crate::data::order_status::OrderStatus;
use crate::data::order_time_in_force::OrderTimeInForce;
use crate::data::order_type::OrderType;

///
/// The `https://www.binance.com/api/v3/order` POST result-type response.
///
/// Result response contains all the order data, except for the partial order fills.
///
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    /// The symbol name.
    pub symbol: String,
    /// The server-side order ID.
    pub order_id: i64,
    /// The client-side order ID.
    pub client_order_id: String,
    /// The time when the order was acknowledged.
    pub transact_time: i64,
    /// The order price.
    pub price: Decimal,
    /// The initial order quantity.
    pub orig_qty: Decimal,
    /// The quantity executed so far. Usually equal to `orig_qty` for market orders.
    pub executed_qty: Decimal,
    /// Usually the same as `executed_qty`.
    pub cummulative_quote_qty: Decimal,
    /// The order status.
    pub status: OrderStatus,
    /// The order time-in-force.
    pub time_in_force: OrderTimeInForce,
    /// The order type.
    pub r#type: OrderType,
    /// The order side.
    pub side: OrderSide,
}
