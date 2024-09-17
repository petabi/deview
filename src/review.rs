use std::{path::Path, sync::Arc};

use anyhow::Result;
use review_database::Store;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct Review {
    _store: Arc<RwLock<Store>>,
}

impl Review {
    pub fn new<R: AsRef<Path>>(data: R, backup: R) -> Result<Self> {
        let store = Arc::new(RwLock::new(Store::new(data.as_ref(), backup.as_ref())?));
        Ok(Self { _store: store })
    }
}
