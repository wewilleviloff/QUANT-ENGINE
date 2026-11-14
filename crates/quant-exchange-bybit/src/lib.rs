//! Bybit V5 integration boundary.
//! Implement public market-data and authenticated private streams in a later, tested phase.
//! Do not place API credentials in this crate's configuration structs or logs.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    Linear,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Environment {
    Testnet,
    Mainnet,
}
