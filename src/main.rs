use dioxus::prelude::*;

mod routes;
use crate::routes::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
    #[layout(Navbar)]
    #[route("/converter")]
    Converter {},
    #[route("/bpm")]
    BPM {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const SAKURA_CSS: &str = "https://cdn.jsdelivr.net/npm/sakura.css@1.5.1/css/sakura-earthly.css";
const MAIN_CSS: Asset = asset!("/assets/style.css");

#[server(endpoint = "static_routes", output = server_fn::codec::Json)]
async fn static_routes() -> Result<Vec<String>, ServerFnError> {
    // The `Routable` trait has a `static_routes` method that returns all static routes in the enum
    Ok(Route::static_routes()
        .iter()
        .map(ToString::to_string)
        .collect())
}

fn main() {
    dioxus::LaunchBuilder::new()
        // Set the server config only if we are building the server target
        .with_cfg(server_only! {
            ServeConfig::builder()
                // Enable incremental rendering
                .incremental(
                    dioxus::server::IncrementalRendererConfig::new()
                        // Store static files in the public directory where other static assets like wasm are stored
                        .static_dir(
                            std::env::current_exe()
                                .unwrap()
                                .parent()
                                .unwrap()
                                .join("public")
                        )
                        // Don't clear the public folder on every build. The public folder has other files including the wasm
                        // binary and static assets required for the app to run
                        .clear_cache(false)
                )
                .enable_out_of_order_streaming()
        })
        .launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: SAKURA_CSS }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        Router::<Route> {}
    }
}

#[component]
fn TitleAndMeta(title: String, description: String) -> Element {
    let title = format!("{title} - naca-nyan.github.io");

    rsx! {
        document::Title { "{title}" }
        document::Meta { name: "og:title", content: "{title}" }
        document::Meta { name: "og:image", content: asset!("/assets/icon.png") }
        document::Meta { name: "twitter:card", content: "summary" }
        document::Meta { name: "twitter:description", content: "{description}" }
        document::Meta { name: "twitter:site", content: "@naca-nyan" }
        document::Meta { name: "twitter:creator", content: "@naca-nyan" }
    }
}

#[component]
fn Navbar() -> Element {
    rsx! {
        nav {
            div { class: "logo-container",
                Link { to: Route::Home {},
                    img { class: "logo", src: asset!("/assets/icon.png") }
                }
            }
        }

        Outlet::<Route> {}
    }
}
