#[cfg(feature = "server")]
use std::{path::Path, sync::Arc};

#[cfg(feature = "server")]
use anyhow::Result;
use dioxus::prelude::*;
#[cfg(feature = "server")]
use review_database::Store;
use serde::{Deserialize, Serialize};
#[cfg(feature = "server")]
use tokio::sync::RwLock;

#[cfg(feature = "server")]
#[derive(Clone)]
pub struct State {
    store: Arc<RwLock<Store>>,
}

#[cfg(feature = "server")]
impl State {
    pub fn new<R: AsRef<Path>>(data: R, backup: R) -> Result<Self> {
        let store = Arc::new(RwLock::new(Store::new(data.as_ref(), backup.as_ref())?));
        Ok(Self { store })
    }

    pub async fn access_tokens(&self) -> Result<Vec<AccessToken>> {
        use review_database::{Direction, Iterable};

        let store = self.store.read().await;
        let table = store.access_token_map();
        Ok(table
            .iter(Direction::Forward, None)
            .filter_map(|res| match res {
                Ok(item) => Some(AccessToken::from(item)),
                Err(e) => {
                    dioxus_logger::tracing::error!("Error retrieving access token: {:?}", e);
                    None
                }
            })
            .collect())
    }
}

#[derive(PartialEq, Props, Clone, Serialize, Deserialize)]
pub struct AccessToken {
    username: String,
    token: String,
}

pub fn AccessTokenEntry(entry: AccessToken) -> Element {
    rsx! {
        h1 { "{entry.username}: ", {entry.token} }
    }
}

#[cfg(feature = "server")]
impl From<review_database::AccessToken> for AccessToken {
    fn from(input: review_database::AccessToken) -> Self {
        Self {
            username: input.username,
            token: input.token,
        }
    }
}

#[server]
pub(crate) async fn access_token_entries() -> Result<Vec<AccessToken>, ServerFnError> {
    let review = review().await?;

    review.access_tokens().await.map_err(ServerFnError::new)
}

#[cfg(feature = "server")]
pub(crate) async fn review() -> Result<State, ServerFnError> {
    use axum::Extension;
    let Extension(review): Extension<State> = extract().await?;
    Ok(review)
}
