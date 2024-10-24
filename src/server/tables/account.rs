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
        p { "{entry}" }
    }
}

#[component]
pub fn Digest() -> Element {
    let entries = use_server_future(accounts)?;
    rsx! {
        tr {
            th { style: "width: 200px; text-align: right;", scope: "row",
                Link {
                    to: crate::Route::Table {
                        name: super::LookUp::Account.to_string(),
                    },
                    "Account"
                }
            }
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

#[component]
fn Columns(entry: String) -> Element {
    let entry: serde_json::Value = serde_json::from_str(&entry).ok()?;

    if let serde_json::Value::Object(entry) = entry {
        let mut entry: Vec<_> = entry.into_iter().map(|(k, _v)| k).collect();
        entry.sort_unstable();
        rsx! {
            tr {
                for k in entry {
                    th { scope: "col", "{k}" }
                }
            }
        }
    } else {
        rsx! {}
    }
}

#[component]
fn Row(entry: String) -> Element {
    let entry: serde_json::Value = serde_json::from_str(&entry).ok()?;
    if let serde_json::Value::Object(entry) = entry {
        let mut entry: Vec<_> = entry.into_iter().collect();
        entry.sort_unstable_by(|a, b| a.0.cmp(&b.0));
        rsx! {
            tr {
                for (_k , v) in entry {
                    td { "{v}" }
                }
            }
        }
    } else {
        rsx! {
            tr {
                td { "{entry}" }
            }
        }
    }
}

#[component]
pub(crate) fn Full() -> Element {
    let entries = use_server_future(accounts)?;
    rsx! {
        table { style: "table-layout: fixed;
                max-width: 100%; max-height: 600px;
                overflow: auto; display: block;
                border-spacing: 0;",
            caption { style: "font: small-caps bold 24px sans-serif; text-align: center; border-bottom: 1px solid rgba(0, 0, 0, 0.5)",
                "Account"
            }
            thead {
                match entries() {
                    None => rsx!{td {colspan: 2, "Loading..."}},
                    Some(Err(e)) => rsx!{td {colspan: 2, "{e}"}},
                    Some(Ok(entries)) => {
                        rsx!{
                            if let Some(entry) = entries.first() {
                                Columns{ entry }
                            }
                            for entry in entries.into_iter() {
                                Row { entry }
                            }
                        }
                    }
                }
            }
        }
    }
}
