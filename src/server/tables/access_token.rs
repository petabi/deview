#[cfg(feature = "server")]
use anyhow::Result;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use crate::server::{state::review, State};

#[cfg(feature = "server")]
impl State {
    pub async fn access_tokens(&self) -> Result<Vec<ATokenProps>> {
        use review_database::{Direction, Iterable};

        let store = self.store.read().await;
        let table = store.access_token_map();
        Ok(table
            .iter(Direction::Forward, None)
            .filter_map(|res| match res {
                Ok(item) => Some(ATokenProps::from(item)),
                Err(e) => {
                    dioxus_logger::tracing::error!("Error retrieving access token: {:?}", e);
                    None
                }
            })
            .collect())
    }
}

#[server]
async fn access_token_entries() -> Result<Vec<ATokenProps>, ServerFnError> {
    let review = review().await?;

    review.access_tokens().await.map_err(ServerFnError::new)
}

#[derive(PartialEq, Props, Clone, Serialize, Deserialize)]
pub struct ATokenProps {
    username: String,
    token: String,
}

#[component]
fn Entry(entry: ATokenProps) -> Element {
    rsx! {
        h1 { style: "word-break: break-word;",
            "{entry.username}: "
            {entry.token}
        }
    }
}

#[component]
fn Row(entry: ATokenProps) -> Element {
    rsx! {
        tr {
            class: "odd:bg-white even:bg-gray-100",
            th { "{entry.username}" }
            td { "{entry.token}" }
        }
    }
}

#[cfg(feature = "server")]
impl From<review_database::AccessToken> for ATokenProps {
    fn from(input: review_database::AccessToken) -> Self {
        Self {
            username: input.username,
            token: input.token,
        }
    }
}

#[component]
pub(crate) fn Digest() -> Element {
    let entries = use_server_future(access_token_entries)?;
    rsx! {
        tr {
            th { style: "width: 200px; text-align: right;", scope: "row",
                Link {
                    class: "hover:bg-gray-100",
                    to: crate::Route::Table {
                        name: super::LookUp::AccessToken.to_string(),
                    },
                    "Access Tokens"
                }
            }
            match entries() {
                None => rsx!{td {colspan: 2, "Loading..."}},
                Some(Err(e)) => rsx!{td {colspan: 2, "{e}"}},
                Some(Ok(entries)) => rsx!{
                    td { style: "width: 100px; text-align: center;", "{entries.len()}" }
                    td {
                        ol {
                            for entry in entries.into_iter().take(3) {
                                li {
                                    Entry { entry: entry.clone() }
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
pub(crate) fn Full() -> Element {
    let entries = use_server_future(access_token_entries)?;
    rsx! {
        table { style: "table-layout: fixed;
                max-width: 1200px; max-height: 1200px;
                overflow: auto; display: block;
                border-spacing: 0;",
            caption { style: "font: small-caps bold 24px sans-serif; text-align: center; border-bottom: 1px solid rgba(0, 0, 0, 0.5)",
                "Access Tokens"
            }
            thead {
                tr { style: "position: sticky; top: 0; background: rgba(0, 0, 0, 0.1);",
                    th {
                        style: "width: 200px; text-align: right;",
                        scope: "col",
                        "User Name"
                    }
                    th { scope: "col", "Token" }
                }
                match entries() {
                    None => rsx!{td {colspan: 2, "Loading..."}},
                    Some(Err(e)) => rsx!{td {colspan: 2, "{e}"}},
                    Some(Ok(entries)) => rsx!{
                        for entry in entries.into_iter() {
                            Row { entry }
                        }
                    }
                }
            }
        }
    }
}
