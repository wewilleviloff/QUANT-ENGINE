//! Pre-trade sizing guardrails. This is an approximation for linear USDT contracts;
//! production sizing must use instrument filters and the exact execution PnL model.

use rust_decimal::Decimal;
use thiserror::Error;

#[derive(Debug, Clone, Copy)]
pub struct SizingInput {
    pub equity: Decimal,
    pub risk_fraction: Decimal,
    pub stop_distance_fraction: Decimal,
    /// Round-trip fees + estimated slippage + funding, as a fraction of notional.
    pub cost_fraction: Decimal,
    pub max_margin_fraction: Decimal,
    pub requested_leverage: Decimal,
    pub max_leverage: Decimal,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SizingResult {
    pub notional: Decimal,
    pub margin: Decimal,
    pub estimated_loss_at_stop: Decimal,
    pub effective_leverage: Decimal,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SizingError {
    #[error("equity must be positive")]
    InvalidEquity,
    #[error("risk fraction must be in (0, 1]")]
    InvalidRiskFraction,
    #[error("stop distance must be positive and cost fraction non-negative")]
    InvalidDistanceOrCosts,
    #[error("margin fraction must be in (0, 1]")]
    InvalidMarginFraction,
    #[error("leverage values must be positive")]
    InvalidLeverage,
    #[error("computed notional is zero")]
    ZeroNotional,
}

pub fn calculate_size(input: SizingInput) -> Result<SizingResult, SizingError> {
    let zero = Decimal::ZERO;
    let one = Decimal::ONE;

    if input.equity <= zero {
        return Err(SizingError::InvalidEquity);
    }
    if input.risk_fraction <= zero || input.risk_fraction > one {
        return Err(SizingError::InvalidRiskFraction);
    }
    if input.stop_distance_fraction <= zero || input.cost_fraction < zero {
        return Err(SizingError::InvalidDistanceOrCosts);
    }
    if input.max_margin_fraction <= zero || input.max_margin_fraction >= one {
        return Err(SizingError::InvalidMarginFraction);
    }
    if input.requested_leverage <= zero || input.max_leverage <= zero {
        return Err(SizingError::InvalidLeverage);
    }

    let effective_leverage = input.requested_leverage.min(input.max_leverage);
    let loss_fraction = input.stop_distance_fraction + input.cost_fraction;
    let risk_limited_notional = input.equity * input.risk_fraction / loss_fraction;
    let margin_limited_notional =
        input.equity * input.max_margin_fraction * effective_leverage;
    let notional = risk_limited_notional.min(margin_limited_notional);

    if notional <= zero {
        return Err(SizingError::ZeroNotional);
    }

    Ok(SizingResult {
        notional,
        margin: notional / effective_leverage,
        estimated_loss_at_stop: notional * loss_fraction,
        effective_leverage,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros_placeholder::*;

    fn d(value: &str) -> Decimal {
        value.parse().expect("valid decimal")
    }

    #[test]
    fn sizes_by_risk_then_applies_margin_cap_and_leverage_cap() {
        let result = calculate_size(SizingInput {
            equity: d("200"),
            risk_fraction: d("0.005"),
            stop_distance_fraction: d("0.02"),
            cost_fraction: d("0.001"),
            max_margin_fraction: d("0.10"),
            requested_leverage: d("5"),
            max_leverage: d("3"),
        })
        .expect("valid sizing");

        assert_eq!(result.effective_leverage, d("3"));
        assert_eq!(result.notional, d("47.619047619047619047619047619"));
        assert_eq!(result.margin, result.notional / d("3"));
        assert!(result.estimated_loss_at_stop <= d("1"));
    }

    #[test]
    fn leverage_does_not_increase_risk_budget() {
        let base = SizingInput {
            equity: d("200"),
            risk_fraction: d("0.005"),
            stop_distance_fraction: d("0.02"),
            cost_fraction: d("0.001"),
            max_margin_fraction: d("0.10"),
            requested_leverage: d("1"),
            max_leverage: d("3"),
        };
        let one_x = calculate_size(base).expect("1x");
        let three_x = calculate_size(SizingInput {
            requested_leverage: d("3"),
            ..base
        })
        .expect("3x");

        assert_eq!(one_x.notional, three_x.notional);
        assert!(three_x.margin < one_x.margin);
        assert_eq!(one_x.estimated_loss_at_stop, three_x.estimated_loss_at_stop);
    }
}
