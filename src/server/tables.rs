mod access_token;
mod account;
use dioxus::prelude::*;

#[component]
pub fn TableDigest() -> Element {
    rsx! {
        table {
            style: "table-layout: fixed;
                max-width: 100%; max-height: 600px;
                overflow: auto; display: block;
                border-spacing: 0;",
            caption {
                style: "font: small-caps bold 24px sans-serif; text-align: center; border-bottom: 1px solid rgba(0, 0, 0, 0.5)",
                    a {
                        href: "https://github.com/petabi/review-database",
                        "REview-database"
                    }
                    " Overview"
            }
            thead {
                tr {
                    style: "position: sticky; top: 0; background: rgba(0, 0, 0, 0.1);",
                    th { style: "width: 200px; text-align: right;", scope: "col", "Table Name" }
                    th { style: "width: 100px; text-align: center;", scope: "col", "Count" }
                    th { scope: "col", "Samples" }
                }
            }
            tbody {
                style: "vertical-align: top;",
                access_token::Digest{}
                account::Digest{}
                super::BackupDigest{}
            }
            tfoot {
                style: "font-color: rgba(0, 0, 0, 0.5); border-top: 1px solid rgba(0, 0, 0, 0.5)",
                tr {
                    th { colspan: 3, super::Info{}}
                }
            }
        }
    }
}
