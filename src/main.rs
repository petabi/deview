#![allow(non_snake_case)]

mod components;
#[cfg(feature = "server")]
mod config;
mod server;

use components::Footer;
use dioxus::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[rustfmt::skip]
enum Route {
    #[layout(NavBar)]
        #[route("/")]
        Home {},
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

fn main() {
    #[cfg(feature = "web")]
    // Hydrate the application on the client
    dioxus_web::launch::launch_cfg(App, dioxus_web::Config::new().hydrate(true));

    #[cfg(feature = "server")]
    {
        use axum::Extension;
        use axum::Router;
        use dioxus_logger::tracing;

        dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
        tracing::info!("starting app");

        let config = config::Config::load_config(parse().as_deref()).expect("fail to load config");
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                let review = config.to_state().expect("unable to open review-database");
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
fn NavBar() -> Element {
    rsx! {
        div {
            class: "divide-y divide-gray-500",
            div {
                nav {
                    ul {
                        li {
                            Link { to: Route::Home {}, "DEVIEW" }
                        }
                    }
                }
                div {
                    a { href: "https://github.com/petabi/deview",
                        svg {
                            class: "bg-transparent translate-x-3 translate-y-3 scale-150",
                            width: "30",
                            height: "40",
                            path {
                                class: "scale-150",
                                d: "M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z"
                            }
                        }
                    }
                }
            }
            Outlet::<Route> {}
        }

        Footer {}
    }
}

#[component]
fn Home() -> Element {
    use server::access_token_entries;
    use server::AccessTokenEntry;

    let mut count = use_signal(|| 0);
    let input_style = r"
        border: none;
        border-radius: 10px;
        box-shadow: 0px 0px 12px 4px rgba(0,0,0,0.74);
    ";

    rsx! {
        h1 { "Counter: {count}" }
        button { style: "{input_style}", onclick: move |_| count += 1, "Up!" }
        button { style: "{input_style}",onclick: move |_| count -= 1, "Down!" }
        button {
            style: "{input_style}",
            onclick: move |_| {
                async move {
                    if let Ok(access_tokens) = access_token_entries().await {
                        count.set(access_tokens.len());
                        for at in access_tokens {
                            AccessTokenEntry(at);
                        }
                    }
                }
            },
            "# of Access Tokens"
        }
    }
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
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
