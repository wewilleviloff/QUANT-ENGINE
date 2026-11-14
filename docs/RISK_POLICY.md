# Risk policy — bootstrap defaults

This document specifies initial guardrails, not an optimized strategy and not a promise of performance.

## Capital and leverage

- Maximum of 3 simultaneous positions.
- Maximum margin allocation per position: 10% of current equity.
- Maximum leverage: 3x. The risk manager may choose lower leverage or reject the trade.
- Initial risk budget: 0.5% of equity per trade.
- Initial aggregate open risk budget: 1.5% of equity.
- Initial daily loss circuit breaker: 2% of UTC-day starting equity.

Margin allocation and risk-at-stop are different quantities. Position notional is first constrained by the allowed loss at the stop (including estimated costs), and only then capped by the allowed margin multiplied by permitted leverage. Leverage must never be used to enlarge the risk budget.

Example at $200 equity: 10% margin allocation is $20. At 3x it can support at most $60 notional, subject to risk sizing, venue minimums and instrument precision. This is a ceiling, not a target.

## Cost model

Pre-trade estimates must include opening and closing fees, expected slippage, and expected funding over the holding horizon. All rates must be expressed on a consistent notional basis. In live accounting, use actual fills, actual fee currency/amount and funding cashflows.

## Expectancy

Net expectancy per trade:

\`E = P(win) × AvgNetWin − P(loss) × AvgNetLoss\`

Win/loss payoffs must be net of commissions, funding and slippage. A score or uncalibrated model confidence is not a win probability. No position may be opened solely to meet a daily activity or return target. A non-positive or unsupported net expectancy is a no-trade result.

## Circuit breakers

Reject new entries when any of the following applies:
- position count or aggregate open risk cap reached;
- daily loss limit reached;
- risk inputs, equity, instrument metadata or stop price are missing/stale;
- notional is below venue minimum or quantity cannot be represented at qtyStep;
- stop/target geometry is invalid;
- estimated net expectancy is not positive;
- exchange/account state cannot be reconciled.

A circuit breaker does not remove the obligation to manage/close existing risk. It blocks new exposure and raises an auditable risk event.

## Validation

The defaults above must be tested against historical data and paper-trading observations. Report drawdown, tail loss, net expectancy confidence intervals, profit factor, turnover, fee-to-gross-profit ratio, MAE/MFE and regime-specific results. Do not optimize solely for win rate or gross return.
