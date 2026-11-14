BEGIN;

CREATE TABLE IF NOT EXISTS instruments (
    symbol              TEXT PRIMARY KEY,
    category            TEXT NOT NULL CHECK (category IN ('linear')),
    base_coin           TEXT NOT NULL,
    quote_coin          TEXT NOT NULL,
    settle_coin         TEXT NOT NULL,
    tick_size           NUMERIC(38, 18) NOT NULL CHECK (tick_size > 0),
    qty_step            NUMERIC(38, 18) NOT NULL CHECK (qty_step > 0),
    min_order_qty       NUMERIC(38, 18) NOT NULL CHECK (min_order_qty >= 0),
    min_notional_value  NUMERIC(38, 18),
    status              TEXT NOT NULL,
    metadata            JSONB NOT NULL DEFAULT '{}'::jsonb,
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS candles (
    symbol       TEXT NOT NULL REFERENCES instruments(symbol),
    interval     TEXT NOT NULL,
    open_time    TIMESTAMPTZ NOT NULL,
    close_time   TIMESTAMPTZ NOT NULL,
    open         NUMERIC(38, 18) NOT NULL,
    high         NUMERIC(38, 18) NOT NULL,
    low          NUMERIC(38, 18) NOT NULL,
    close        NUMERIC(38, 18) NOT NULL,
    volume       NUMERIC(38, 18) NOT NULL,
    turnover     NUMERIC(38, 18),
    is_closed    BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY (symbol, interval, open_time),
    CHECK (high >= low),
    CHECK (volume >= 0)
);

CREATE TABLE IF NOT EXISTS signals (
    id                  UUID PRIMARY KEY,
    symbol              TEXT NOT NULL REFERENCES instruments(symbol),
    strategy_id         TEXT NOT NULL,
    side                TEXT NOT NULL CHECK (side IN ('buy', 'sell')),
    entry_reference     NUMERIC(38, 18) NOT NULL,
    stop_price          NUMERIC(38, 18) NOT NULL,
    take_profit_price   NUMERIC(38, 18) NOT NULL,
    win_probability     NUMERIC(12, 10),
    expected_value      NUMERIC(38, 18),
    status              TEXT NOT NULL DEFAULT 'created',
    payload             JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS positions (
    id                  UUID PRIMARY KEY,
    symbol              TEXT NOT NULL REFERENCES instruments(symbol),
    side                TEXT NOT NULL CHECK (side IN ('buy', 'sell')),
    quantity            NUMERIC(38, 18) NOT NULL CHECK (quantity >= 0),
    average_entry_price NUMERIC(38, 18) NOT NULL,
    leverage            NUMERIC(12, 6) NOT NULL CHECK (leverage > 0 AND leverage <= 3),
    margin_used         NUMERIC(38, 18) NOT NULL CHECK (margin_used >= 0),
    stop_order_id       TEXT,
    take_profit_order_id TEXT,
    status              TEXT NOT NULL,
    opened_at           TIMESTAMPTZ NOT NULL,
    closed_at           TIMESTAMPTZ
);

CREATE UNIQUE INDEX IF NOT EXISTS positions_one_open_per_symbol
    ON positions(symbol) WHERE status = 'open';

CREATE TABLE IF NOT EXISTS orders (
    id                  UUID PRIMARY KEY,
    client_order_id     TEXT NOT NULL UNIQUE,
    exchange_order_id   TEXT,
    signal_id           UUID REFERENCES signals(id),
    position_id         UUID REFERENCES positions(id),
    symbol              TEXT NOT NULL REFERENCES instruments(symbol),
    side                TEXT NOT NULL CHECK (side IN ('Buy', 'Sell')),
    order_type          TEXT NOT NULL CHECK (order_type IN ('Market', 'Limit')),
    quantity            NUMERIC(38, 18) NOT NULL CHECK (quantity > 0),
    price               NUMERIC(38, 18),
    reduce_only         BOOLEAN NOT NULL DEFAULT FALSE,
    status              TEXT NOT NULL,
    mode                TEXT NOT NULL CHECK (mode IN ('paper', 'demo', 'live')),
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS executions (
    id                  UUID PRIMARY KEY,
    order_id            UUID NOT NULL REFERENCES orders(id),
    exchange_execution_id TEXT UNIQUE,
    price               NUMERIC(38, 18) NOT NULL,
    quantity            NUMERIC(38, 18) NOT NULL CHECK (quantity > 0),
    fee                 NUMERIC(38, 18) NOT NULL DEFAULT 0,
    fee_currency        TEXT,
    executed_at         TIMESTAMPTZ NOT NULL,
    raw_payload         JSONB NOT NULL DEFAULT '{}'::jsonb
);

CREATE TABLE IF NOT EXISTS equity_snapshots (
    id              UUID PRIMARY KEY,
    equity          NUMERIC(38, 18) NOT NULL,
    available_balance NUMERIC(38, 18),
    unrealized_pnl  NUMERIC(38, 18) NOT NULL DEFAULT 0,
    currency        TEXT NOT NULL DEFAULT 'USDT',
    captured_at     TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS risk_events (
    id              UUID PRIMARY KEY,
    event_type      TEXT NOT NULL,
    severity        TEXT NOT NULL,
    symbol          TEXT,
    decision        TEXT NOT NULL,
    reason          TEXT NOT NULL,
    context         JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS candles_symbol_interval_time
    ON candles(symbol, interval, open_time DESC);
CREATE INDEX IF NOT EXISTS signals_created_at
    ON signals(created_at DESC);
CREATE INDEX IF NOT EXISTS orders_status_updated_at
    ON orders(status, updated_at DESC);
CREATE INDEX IF NOT EXISTS executions_order_time
    ON executions(order_id, executed_at);

COMMIT;
