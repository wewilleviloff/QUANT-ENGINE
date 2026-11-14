# Architecture

## Runtime ownership

Rust is the only process permitted to own an exchange execution adapter. Python is a research and analytics environment and has no order-submission interface. The dashboard is a client of the Rust API and must never receive exchange credentials.

## Runtime flow (target)

1. Exchange market data is normalized into domain candles/trades/order-book events.
2. Feature calculations are timestamped and versioned.
3. Regime detection classifies trend/range/volatility/liquidity conditions.
4. Strategies produce candidate signals; they do not size positions or send orders.
5. Risk manager independently validates signal, portfolio exposure, equity, stop distance, costs and exchange constraints.
6. Execution engine creates idempotent commands and reconciles orders, fills and positions.
7. Accounting derives realized and unrealized PnL from fills, fees, funding and cash movements.
8. Persistent audit records support replay and post-trade review.

## Rust boundaries

- \`quant-domain\`: exchange-neutral data types.
- \`quant-market-data\`: normalized market events and candle models.
- \`quant-strategy\`: strategy interfaces; no venue access.
- \`quant-risk\`: position sizing and portfolio risk policy.
- \`quant-execution\`: order lifecycle and submission boundary.
- \`quant-exchange-bybit\`: Bybit V5 transport and venue-specific models (future implementation).
- \`quant-accounting\`: fill-based PnL and net expectancy.
- \`quant-persistence\`: PostgreSQL repositories and transactions.
- \`quant-api\`: read/control API; current bootstrap exposes health only.
- \`quant-telemetry\`: structured logging and later metrics/tracing.

## Quant Lab

Python is intended for historical data ingestion, feature research, labels, event-driven backtests, walk-forward validation, statistical reporting and model experiments. Research artifacts must include dataset, code, parameter and model versions. No Python package may send orders.

## Operational state

PostgreSQL is the source of truth for orders, executions, positions, risk events and account snapshots. Parquet is the preferred immutable historical-data format; DuckDB is a local/analytical query engine, not the live account ledger.

## Reliability requirements before demo/live

- Private WebSocket reconnect and REST reconciliation.
- Idempotent client order identifiers.
- Partial-fill and duplicate-event handling.
- Confirmed protective stop after entry; flatten or escalate if protection cannot be confirmed.
- Exchange constraints loaded from instrument metadata at startup and refreshed.
- Safe startup recovery from open orders, fills, positions and wallet state.
- Explicit operator-controlled mode changes; no automatic promotion from paper to demo/live.
