# QUANT ENGINE

A paper-first, risk-controlled quantitative trading system. Rust owns live execution; Python is research-only.

> **Safety:** Live trading is disabled by default. Never commit API keys, exchange secrets, or account identifiers.

## Repository status

Bootstrap scaffold — architecture and contracts are established; exchange connectivity, strategy logic, and execution are intentionally not enabled yet.

## Architecture

- **Rust workspace:** trading runtime, domain types, market data, strategy pipeline, risk checks, execution boundary, accounting, persistence, API, telemetry.
- **Python Quant Lab:** historical data, feature research, backtesting, validation, model experiments, trade review. It cannot place orders.
- **PostgreSQL:** operational state and audit trail.
- **Parquet + DuckDB:** research datasets and analytical workloads.
- **SvelteKit dashboard:** planned; not part of the initial bootstrap.

See [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md), [docs/RISK_POLICY.md](docs/RISK_POLICY.md), and [docs/DEVELOPMENT.md](docs/DEVELOPMENT.md).

## Safety defaults

- Mode: paper
- Live order submission: disabled
- Maximum concurrent positions: 3
- Maximum margin allocation per position: 10% of current equity
- Maximum leverage: 3x (only when approved by risk checks)
- Initial risk budget: 0.5% equity per trade; 1.5% aggregate open risk
- Initial daily loss circuit breaker: 2% of UTC-day starting equity
- Position sizing is calculated from stop-loss risk first; leverage is not a risk budget.
- No trade is required: a missing or non-positive net expectancy means no entry.

These are conservative starting controls, not return or profitability guarantees. All parameters require historical validation and paper trading.

## Quick start

Requirements: Rust stable, Python 3.12+, Docker Compose v2, and make (optional).

    cp .env.example .env
    docker compose up -d postgres

Rust checks:

    cargo fmt --all -- --check
    cargo check --workspace
    cargo test --workspace

Python Quant Lab:

    python -m venv .venv
    # Windows PowerShell: .venv\Scripts\Activate.ps1
    # macOS/Linux: source .venv/bin/activate
    python -m pip install -e ".[dev]"
    pytest
    ruff check .

The initial Rust binary is a health/readiness shell only. It does not connect to an exchange or place orders. Database migrations are in migrations/; see the development guide before applying them.

## Project layout

    apps/trading-engine/    Rust process entry point
    crates/                 Rust domain and service boundaries
    quant-lab/              Python research package
    contracts/              Versioned JSON Schemas and examples
    migrations/             PostgreSQL migrations
    config/                 Non-secret TOML configuration
    docs/                   Architecture, risk policy, development guide
    infra/                  Local development infrastructure

## License

TBD. Do not assume this repository grants rights to redistribute or commercially use third-party dependencies.
