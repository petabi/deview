#[cfg(feature = "server")]
use std::{path::Path, sync::Arc};

#[cfg(feature = "server")]
use anyhow::{Context, Result};
use dioxus::prelude::*;
#[cfg(feature = "server")]
use review_database::Store;
use serde::{Deserialize, Serialize};
#[cfg(feature = "server")]
use tokio::sync::RwLock;

#[cfg(feature = "server")]
#[derive(Clone)]
pub struct State {
    pub(crate) store: Arc<RwLock<Store>>,
    version: String,
}

#[cfg(feature = "server")]
impl State {
    pub fn new<R: AsRef<Path>>(data: R, backup: R) -> Result<Self> {
        let store = Arc::new(RwLock::new(Store::new(data.as_ref(), backup.as_ref())?));
        let version = data.as_ref().join("VERSION");
        let version = Self::read_version_file(version.as_path())?;
        Ok(Self { store, version })
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    fn read_version_file(path: &Path) -> Result<String> {
        use std::fs::File;
        use std::io::Read;

        let mut ver = String::new();
        File::open(path)
            .context("cannot open VERSION")?
            .read_to_string(&mut ver)
            .context("cannot read VERSION")?;
        Ok(ver)
    }

    pub async fn backups(&self) -> Result<Vec<BackupEngineInfoProps>> {
        let store = self.store.read().await;
        Ok(store
            .get_backup_info()?
            .into_iter()
            .map(From::from)
            .collect())
    }
}

#[cfg(feature = "server")]
pub(crate) async fn review() -> Result<State, ServerFnError> {
    use axum::Extension;
    let Extension(review): Extension<State> = extract().await?;
    Ok(review)
}

#[component]
pub fn BackupDigest() -> Element {
    let entries = use_server_future(backups)?;
    rsx! {
        tr {
            th { style: "width: 200px; text-align: right;", scope: "row", "Backup"}
            match entries() {
                None => rsx!{td { colspan: 2, "Loading..." }},
                Some(Err(e)) => rsx!{td {colspan: 2, "{e}"}},
                Some(Ok(entries)) => rsx!{
                    td { style: "width: 100px; text-align: center;", "{entries.len()}" }
                    td {
                        ol {
                            for entry in entries.into_iter().take(3) {
                                li {
                                    Backup { entry: entry }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Backup(entry: BackupEngineInfoProps) -> Element {
    rsx! {
        div {
            p {
                "id {entry.backup_id}: size {entry.size}, containing {entry.num_files} files at {entry.timestamp};"
            }
        }
    }
}

#[server]
async fn backups() -> Result<Vec<BackupEngineInfoProps>, ServerFnError> {
    let review = review().await?;

    review.backups().await.map_err(ServerFnError::new)
}

#[derive(PartialEq, Props, Clone, Serialize, Deserialize)]
pub struct BackupEngineInfoProps {
    /// Timestamp of the backup
    pub timestamp: i64,
    /// ID of the backup
    pub backup_id: u32,
    /// Size of the backup
    pub size: u64,
    /// Number of files related to the backup
    pub num_files: u32,
}

#[cfg(feature = "server")]
impl From<review_database::BackupEngineInfo> for BackupEngineInfoProps {
    fn from(input: review_database::BackupEngineInfo) -> Self {
        Self {
            timestamp: input.timestamp,
            backup_id: input.backup_id,
            size: input.size,
            num_files: input.num_files,
        }
    }
}

#[component]
pub(crate) fn Digest() -> Element {
    let version = use_server_future(version)?;
    rsx! {
        p {
            "version: ",
            match version() {
                Some(Ok(v)) => v,
                _ => "N/A".to_string(),
            }
        }
    }
}

#[server]
async fn version() -> Result<String, ServerFnError> {
    let review = review().await?;
    Ok(review.version().to_string())
}
