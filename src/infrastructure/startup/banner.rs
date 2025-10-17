use tracing::info;

/// Component health status
#[derive(Debug, Clone, Copy)]
pub struct ComponentHealth {
    pub database: bool,
}

/// Startup banner display
pub struct StartupBanner;

impl StartupBanner {
    /// Displays a formatted startup banner with component health status
    pub fn display(health: ComponentHealth) {
        let app_name = env!("CARGO_PKG_NAME");
        let app_version = env!("CARGO_PKG_VERSION");

        let banner = format!(
            "{top}\n {name} v{ver}\n {dash}\n Startup Summary:\n  - Database: {db}\n{top}",
            top = "+-------------------------------------------------------------+",
            name = app_name,
            ver = app_version,
            dash = "-------------------------------------------------------------",
            db = Self::status_text(health.database),
        );

        info!(banner = %banner, "application initialized");
    }

    fn status_text(healthy: bool) -> &'static str {
        if healthy { "✓ OK" } else { "✗ UNAVAILABLE" }
    }
}
