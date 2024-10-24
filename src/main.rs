#![allow(non_snake_case)]

mod components;
#[cfg(feature = "server")]
mod config;
mod server;

use dioxus::prelude::*;

use crate::components::PageNotFound;
use crate::server::Table;

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[rustfmt::skip]
enum Route {
    #[layout(components::Wrapper)]
        #[route("/")]
        Home {},
        #[route("/table/:name")]
        Table { name: String },
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

fn main() {
    #[cfg(feature = "web")]
    // Hydrate the application on the client
    dioxus::prelude::LaunchBuilder::new()
        .with_cfg(web!(dioxus_web::Config::new().hydrate(true)))
        .launch(App);

    #[cfg(feature = "server")]
    {
        use axum::Extension;
        use axum::Router;
        use dioxus_logger::tracing;

        if let Err(e) = dioxus_logger::init(tracing::Level::INFO) {
            println!("failed to initiate logger {e:?}");
            std::process::exit(1);
        }
        tracing::info!("starting app");

        let config = match config::Config::load_config(parse().as_deref()) {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("failed to load config: {e}");
                std::process::exit(1);
            }
        };

        let review = match config.to_state() {
            Ok(state) => state,
            Err(e) => {
                tracing::error!("failed to load state: {}", e);
                std::process::exit(1);
            }
        };
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                let app = Router::new()
                    // Server side render the application, serve static assets, and register server functions
                    .serve_dioxus_application(ServeConfig::builder().build(), move || {
                        VirtualDom::new(App)
                    })
                    .await
                    .layer(Extension(review));
                let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8080));
                let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

                axum::serve(listener, app.into_make_service())
                    .await
                    .unwrap();
            });
    }
}

#[cfg(any(feature = "server", feature = "web"))]
fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        div { class: "flex min-h-screen flex-col justify-center overflow-scroll",
            div {
                class: "rounded-3xl bg-white shadow-xl ring-1 ring-gray-900/5",
                style: "margin: 2%; padding: 5%; overflow: scroll;",
                server::TableDigest {}
            }
        }
    }
}

#[cfg(feature = "server")]
fn parse() -> Option<String> {
    use std::process::exit;
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() <= 1 {
        return None;
    }

    if args[1] == "--help" || args[1] == "-h" {
        println!("{} {}", package(), version());
        println!();
        println!(
            "USAGE: \
            \n    {} [CONFIG] \
            \n \
            \nFLAGS: \
            \n    -h, --help       Prints help information \
            \n    -V, --version    Prints version information \
            \n \
            \nARG: \
            \n    <CONFIG>    A TOML config file",
            package()
        );
        exit(0);
    }
    if args[1] == "--version" || args[1] == "-V" {
        println!("{}", version());
        exit(0);
    }

    Some(args[1].clone())
}

#[cfg(feature = "server")]
fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(feature = "server")]
fn package() -> &'static str {
    env!("CARGO_PKG_NAME")
}
