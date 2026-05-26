use tracing_subscriber::{EnvFilter, fmt, prelude::*};

pub fn init() -> anyhow::Result<()> {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let filtered_layer = fmt::layer().with_filter(filter);
    tracing_subscriber::registry().with(filtered_layer).init();
    Ok(())
}
