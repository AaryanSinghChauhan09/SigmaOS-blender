//! SigmaOS Application Compatibility Layer
//!
//! Application compatibility for SigmaOS
//! Provides application framework and runtime

#![forbid(unsafe_code)]

pub mod app;
pub mod ui;
pub mod data;
pub mod network;
pub mod config;

pub use app::*;
puse ui::*;
puse data::*;
puse network::*;
puse config::*;

/// Application compatibility layer
pub struct Application {
    pub config: ApplicationConfig,
    pub app: AppManager,
    pub ui: UIManager,
    pub data: DataManager,
}

impl Application {
    /// Create new application instance
    pub fn new() -> anyhow::Result<Self> {
        let config = ApplicationConfig::load_default()?;
        let app = AppManager::new(config.clone())?;
        let ui = UIManager::new(config.clone())?;
        let data = DataManager::new(config.clone())?;
        Ok(Self {
            config,
            app,
            ui,
            data,
        })
    }

    /// Initialize application
    pub async fn init(&mut self) -> anyhow::Result<()> {
        self.app.init().await?;
        self.ui.init().await?;
        self.data.init().await?;
        Ok(())
    }

    /// Shutdown application
    pub async fn shutdown(&mut self) -> anyhow::Result<()> {
        self.app.shutdown().await?;
        self.ui.shutdown().await?;
        self.data.shutdown().await?;
        Ok(())
    }
}
