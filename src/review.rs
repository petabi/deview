mod auth;

use std::{path::Path, sync::Arc};

use anyhow::Result;
use review_database::{Database, Store};
use tokio::sync::RwLock;

pub use self::auth::SignIn;

#[derive(Clone)]
pub struct Review {
    #[allow(clippy::used_underscore_binding)]
    _db: Database,
    store: Arc<RwLock<Store>>,
}

impl Review {
    #[allow(clippy::used_underscore_binding)]
    pub async fn new<
        'a,
        R: AsRef<Path>,
        P: AsRef<Path> + 'a,
        S: AsRef<str>,
        I: Iterator<Item = &'a P>,
    >(
        data: R,
        backup: R,
        url: S,
        certs: I,
    ) -> Result<Self> {
        let store = Arc::new(RwLock::new(Store::new(data.as_ref(), backup.as_ref())?));
        let certs: Vec<_> = certs.map(std::convert::AsRef::as_ref).collect();
        let _db = Database::new(url.as_ref(), &certs).await?;
        Ok(Self { _db, store })
    }
}
