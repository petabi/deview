use std::{env, path::PathBuf};

use anyhow::{Context, Result};
use config::{Environment, File};
use serde::Deserialize;

const DEFAULT_DATABASE_URL: &str = "postgres://review@localhost/review";

#[derive(Debug, Deserialize)]
pub struct Config {
    data_dir: PathBuf,
    backup_dir: PathBuf,
    database_url: String,
    ca_certs: Vec<PathBuf>,
}

impl Config {
    /// Reads configuration from the file on disk and environment variables and
    /// returns Config struct.
    ///
    /// # Errors
    ///
    /// If input arguments are invalid, an error will be returned.
    pub fn load_config(path: Option<&str>) -> Result<Self> {
        let builder = config::Config::builder()
            .set_default("data_dir", env::current_dir()?.join("data").to_str())
            .context("cannot set the default data directory")?
            .set_default("backup_dir", env::current_dir()?.join("backup").to_str())
            .context("cannot set the default backup directory")?
            .set_default("database_url", DEFAULT_DATABASE_URL)
            .context("cannot set the default database URL")?;
        let config: Config = if let Some(path) = path {
            builder.add_source(File::with_name(path))
        } else {
            builder
        }
        .add_source(Environment::with_prefix("REVIEW"))
        .build()
        .context("cannot build the config")?
        .try_deserialize()?;
        Ok(config)
    }

    pub async fn to_review(&self) -> Result<crate::review::Review> {
        crate::review::Review::new(
            &self.data_dir,
            &self.backup_dir,
            &self.database_url,
            self.ca_certs.iter(),
        )
        .await
    }
}
