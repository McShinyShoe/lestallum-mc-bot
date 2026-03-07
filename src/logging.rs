use tracing_subscriber::{EnvFilter, fmt};

pub fn init_tracing(level: String) {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(level));
    fmt().with_env_filter(filter).init();
}
