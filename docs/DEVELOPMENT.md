# Development guide

## Prerequisites

- Rust stable toolchain (edition 2024).
- Python 3.12+.
- Docker Engine/Desktop with Compose v2.
- VS Code with rust-analyzer, Python and Ruff extensions (recommended).

## Local bootstrap

1. Clone the repository and open its root folder in VS Code.
2. Copy \`.env.example\` to \`.env\`; keep it local.
3. Start PostgreSQL: \`docker compose up -d postgres\`.
4. Start the health service: \`cargo run -p trading-engine\`.
5. Check \`http://127.0.0.1:8080/health\` and \`/ready\`.
6. Apply SQL migration 0001 only to a disposable/local development database. A migration runner is not yet implemented.

## Rust checks

\`\`\`bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -p trading-engine
\`\`\`

## Python checks

\`\`\`bash
python -m venv .venv
# Windows PowerShell: .venv\\\\Scripts\\\\Activate.ps1
# macOS/Linux: source .venv/bin/activate
python -m pip install -e ".[dev]"
pytest
ruff check .
\`\`\`

Install research extras only when working on historical datasets: \`python -m pip install -e ".[research,dev]"\`.

## Current limitations

This commit is a scaffold, not a trading bot. It has no exchange connection, no strategy that emits orders, no database repository, no dashboard and no live-order path. Do not add production credentials or enable live trading as a shortcut. Build market-data ingestion and deterministic backtests first; proceed to Bybit testnet only after contract, sizing and reconciliation tests pass.
