//! Execution boundary. No live order transport is implemented in this scaffold.

use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ExecutionError {
    #[error("live trading is disabled in this build")]
    LiveTradingDisabled,
    #[error("paper executor is not configured")]
    PaperExecutorNotConfigured,
}

pub fn assert_live_trading_disabled() -> Result<(), ExecutionError> {
    Err(ExecutionError::LiveTradingDisabled)
}
