//! Expectancy inputs are net of fees, funding and slippage.

use rust_decimal::Decimal;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ExpectancyError {
    #[error("win probability must be between 0 and 1")]
    InvalidProbability,
    #[error("average net win and loss must be non-negative")]
    InvalidPayoff,
}

pub fn net_expectancy(
    win_probability: Decimal,
    average_net_win: Decimal,
    average_net_loss: Decimal,
) -> Result<Decimal, ExpectancyError> {
    if !(Decimal::ZERO..=Decimal::ONE).contains(&win_probability) {
        return Err(ExpectancyError::InvalidProbability);
    }
    if average_net_win < Decimal::ZERO || average_net_loss < Decimal::ZERO {
        return Err(ExpectancyError::InvalidPayoff);
    }

    let loss_probability = Decimal::ONE - win_probability;
    Ok(win_probability * average_net_win - loss_probability * average_net_loss)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(value: &str) -> Decimal {
        value.parse().expect("valid decimal")
    }

    #[test]
    fn calculates_net_expectancy() {
        assert_eq!(
            net_expectancy(d("0.55"), d("2"), d("1")).unwrap(),
            d("0.65")
        );
    }

    #[test]
    fn rejects_probability_outside_unit_interval() {
        assert_eq!(
            net_expectancy(d("1.1"), d("2"), d("1")),
            Err(ExpectancyError::InvalidProbability)
        );
    }
}
