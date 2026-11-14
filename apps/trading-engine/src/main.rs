use std::{env, net::SocketAddr};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    quant_telemetry::init();

    let bind_addr: SocketAddr = env::var("QUANT_BIND_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:8080".to_owned())
        .parse()?;

    let listener = tokio::net::TcpListener::bind(bind_addr).await?;
    tracing::info!(
        address = %listener.local_addr()?,
        mode = "paper",
        live_trading_enabled = false,
        "Quant Engine bootstrap service started"
    );

    axum::serve(listener, quant_api::router()).await?;
    Ok(())
}
