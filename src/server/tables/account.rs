#[cfg(feature = "server")]
use anyhow::Result;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::server::{state::review, State};

#[cfg(feature = "server")]
impl State {
    pub async fn accounts(&self) -> Result<Vec<review_database::types::Account>> {
        use review_database::{Direction, Iterable};

        let store = self.store.read().await;
        let table = store.account_map();
        Ok(table
            .iter(Direction::Forward, None)
            .filter_map(|res| match res {
                Ok(item) => Some(item),
                Err(e) => {
                    dioxus_logger::tracing::error!("Error retrieving account: {:?}", e);
                    None
                }
            })
            .collect())
    }
}

#[server]
async fn accounts() -> Result<Vec<String>, ServerFnError> {
    let review = review().await?;

    review
        .accounts()
        .await
        .map(|entries| {
            entries
                .into_iter()
                .filter_map(|e| serde_json::to_string(&e).ok())
                .collect()
        })
        .map_err(ServerFnError::new)
}

#[component]
fn Account(entry: String) -> Element {
    rsx! {
        p {
            "{entry}"
        }
    }
}

#[component]
pub fn Digest() -> Element {
    let entries = use_server_future(accounts)?;
    rsx! {
        tr {
            th { style: "width: 200px; text-align: right;", scope: "row", "Account"}
            match entries() {
                None => rsx!{td { colspan: 2, "Loading..." }},
                Some(Err(e)) => rsx!{td {colspan: 2, "{e}"}},
                Some(Ok(entries)) => rsx!{
                    td { style: "width: 100px; text-align: center;", "{entries.len()}" }
                    td {
                        ol {
                            for entry in entries.into_iter().take(3) {
                                li {
                                    Account { entry: entry }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
