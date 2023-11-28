use std::str::FromStr;

use color_eyre::Result;
use tracing_error::ErrorLayer;
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter};
use tracing_tree::HierarchicalLayer;

const DEFAULT_ENV_CONFIG: &str = "info,sqlx=error,aoc_sx=debug";

pub struct Logging;

impl Logging {
    pub fn setup() -> Result<()> {
        let log_config =
            std::env::var("RUST_LOG").unwrap_or_else(|_| DEFAULT_ENV_CONFIG.to_string());

        // Capture "log" messages
        LogTracer::init()?;

        let filter_layer = EnvFilter::from_str(&log_config)?;
        let hierarchical_layer = HierarchicalLayer::new(2)
            .with_targets(true)
            .with_bracketed_fields(true);
        let error_layer = ErrorLayer::default();

        tracing::subscriber::set_global_default(
            tracing_subscriber::registry()
                .with(error_layer)
                .with(hierarchical_layer)
                .with(filter_layer),
        )?;

        Ok(())
    }
}
