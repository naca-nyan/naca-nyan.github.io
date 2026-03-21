use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
    #[layout(Navbar)]
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

struct SNS<'a> {
    name: &'a str,
    href: &'a str,
    img: Asset,
}

#[component]
fn Home() -> Element {
    let sns_list = [
        SNS {
            name: "twitter",
            href: "https://twitter.com/naca_nyan",
            img: asset!("/assets/twitter.svg"),
        },
        SNS {
            name: "github",
            href: "https://github.com/naca-nyan",
            img: asset!("/assets/github.svg"),
        },
        SNS {
            name: "vrchat",
            href: "https://vrchat.com/home/user/usr_08cf7780-e093-4e29-a083-2bc7e4957247",
            img: asset!("/assets/vrchat.png"),
        },
        SNS {
            name: "youtube",
            href: "https://www.youtube.com/@naca_nyan",
            img: asset!("/assets/youtube.svg"),
        },
        SNS {
            name: "skeb",
            href: "https://skeb.jp/@naca_nyan",
            img: asset!("/assets/skeb.png"),
        },
    ];
    rsx! {
        TitleAndMeta { title: "Home", description: "なかにゃんのサイトだよ" }
        header {
            div { class: "image-cropper",
                img { class: "profile-pic", src: asset!("/assets/icon.png") }
            }
            div { class: "name", "なかにゃん @naca_nyan" }
            div { class: "sns-list",
                for s in sns_list {
                    a { key: "{s.name}", href: s.href,
                        img { class: "sns-icon", src: s.img }
                    }
                }
            }
        }
        h1 { "便利なやつら" }
        hr {}
        p { "作ったやつ" }
        ul {
            li {
                Link { to: Route::BPM {}, "ぽちぽちしてBPM計測するやつ" }
            }
        }

    }
}

struct BPMCalculator {
    ts: Vec<i64>,
}

impl BPMCalculator {
    fn new() -> Self {
        Self { ts: vec![] }
    }
    fn tap(&mut self) {
        self.ts.push(chrono::Utc::now().timestamp_millis());
    }
    fn reset(&mut self) {
        self.ts.clear();
    }
    fn diff(&self) -> impl Iterator<Item = i64> + '_ {
        self.ts.windows(2).map(|v| v[1] - v[0])
    }
    fn average(&self) -> Option<Option<f64>> {
        let len = self.ts.len();
        if len == 0 {
            None
        } else if self.ts.len() < 2 {
            Some(None)
        } else {
            let diff_sum: i64 = self.diff().sum();
            let diff_len = len - 1;
            Some(Some(diff_sum as f64 / diff_len as f64))
        }
    }
}

fn ms_to_bpm(ms: f64) -> f64 {
    60.0 * 1000.0 / ms
}

#[component]
fn BPM() -> Element {
    let mut bpm = use_signal(BPMCalculator::new);
    let mut str = use_signal(String::new);

    rsx! {
        TitleAndMeta {
            title: "BPM計るやつ",
            description: "ぽちぽちしてBPM計るやつ",
        }
        h1 { "BPM計るやつ" }
        hr {}
        main {
            p {
                label { "↓ボタンをタップ、もしくはテキストエリアでスペース" }
                button { onclick: move |_| bpm.write().tap(), "Tap" }
                input {
                    value: "{str}",
                    oninput: move |e| {
                        bpm.write().tap();
                        *str.write() = e.value();
                    },
                }
                button {
                    onclick: move |_| {
                        bpm.write().reset();
                        str.write().clear();
                    },
                    "Reset"
                }
            }
            if let Some(avg) = bpm.read().average() {
                if let Some(ms) = avg {
                    div { "Average: {ms_to_bpm(ms)} ({ms}ms)" }
                } else {
                    div { "First Tap" }
                }
            }
            ol {
                for ms in bpm.read().diff() {
                    li { "{ms_to_bpm(ms as f64)} ({ms}ms)" }
                }
            }
        }
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
