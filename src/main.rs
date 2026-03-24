use dioxus::prelude::*;

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
                Link { to: Route::Converter {}, "いろいろ変換するやつ" }
            }
            li {
                Link { to: Route::BPM {}, "ぽちぽちしてBPM計測するやつ" }
            }
        }

    }
}

#[derive(Clone, Copy)]
struct MinSec(f64);

impl From<String> for MinSec {
    fn from(value: String) -> Self {
        let v = value
            .split(":")
            .map(|s| s.parse().unwrap_or_default())
            .fold(0.0, |acc, x: f64| acc * 60.0 + x);
        MinSec(v)
    }
}

impl std::fmt::Display for MinSec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02}:{:02}",
            (self.0 / 60.0).floor(),
            (self.0 % 60.0).round()
        )
    }
}

impl std::ops::Mul<f64> for MinSec {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs)
    }
}

#[component]
fn Converter() -> Element {
    let mut percent = use_signal(|| 100.0);
    let mut semitone = use_signal(|| 0.0);

    let mut set_percent = move |v: f64| {
        *percent.write() = v;
        *semitone.write() = (v / 100.0).log2() * 12.0;
    };

    let mut set_semitone = move |v: f64| {
        *percent.write() = 2.0_f64.powf(v / 12.0) * 100.0;
        *semitone.write() = v;
    };

    let mut ms = use_signal(|| 500.0);
    let mut set_ms = move |v| *ms.write() = v;
    let hz = 1000.0 / ms();
    let mut set_hz = move |v: f64| *ms.write() = 1000.0 / v;
    let bpm = hz * 60.0;
    let mut set_bpm = move |v: f64| set_hz(v / 60.0);

    let mut watt1 = use_signal(|| 500.0);
    let mut time1 = use_signal(|| MinSec::from("02:30".to_string()));
    let mut watt2 = use_signal(|| 800.0);
    let time2 = time1() * (watt1() / watt2());

    rsx! {
        TitleAndMeta {
            title: "いろいろ変換するやつ",
            description: "べんりな数値変換ツールたち",
        }

        h1 { "いろいろ変換するやつ" }
        hr {}
        main {
            h4 { "ボイチェンの％と半音単位こんばーた" }
            p {
                label { "パーセント" }
                input {
                    r#type: "number",
                    value: "{percent}",
                    oninput: move |e| set_percent(e.parsed().unwrap_or_default()),
                }
                "%"
                label { "半音単位" }
                input {
                    r#type: "number",
                    value: "{semitone}",
                    oninput: move |e| set_semitone(e.parsed().unwrap_or_default()),
                }
                "st"
            }
            hr {}
            h4 { "HzとBPMとms行ったり来たり" }
            p {
                label { "Hz" }
                input {
                    r#type: "number",
                    value: hz,
                    oninput: move |e| set_hz(e.parsed().unwrap_or_default()),
                }
                label { "BPM" }
                input {
                    r#type: "number",
                    value: bpm,
                    oninput: move |e| set_bpm(e.parsed().unwrap_or_default()),
                }
                label { "ms" }
                input {
                    r#type: "number",
                    value: ms,
                    oninput: move |e| set_ms(e.parsed().unwrap_or_default()),
                }
            }
            hr {}
            h4 { "電子レンジ秒数くん" }
            p {
                label {}
                input {
                    r#type: "number",
                    step: 100,
                    value: watt1,
                    oninput: move |e| *watt1.write() = e.parsed().unwrap_or_default(),
                }
                "W の"
                input {
                    r#type: "time",
                    step: 600,
                    value: "{time1}",
                    oninput: move |e| *time1.write() = MinSec::from(e.value()),
                }
                "は"
                label {}
                input {
                    r#type: "number",
                    step: 100,
                    value: watt2,
                    oninput: move |e| *watt2.write() = e.parsed().unwrap_or_default(),
                }
                "W の"
                input {
                    r#type: "time",
                    step: 600,
                    value: "{time2}",
                    readonly: true,
                }
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
        self.ts.windows(2).map(|v| v[1] - v[0]).rev()
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

    rsx! {
        TitleAndMeta {
            title: "BPM計るやつ",
            description: "ぽちぽちしてBPM計るやつ",
        }
        h1 { "BPM計るやつ" }
        hr {}
        main {
            p {
                button {
                    onmounted: async move |e| e.set_focus(true).await.unwrap_or_default(),
                    onclick: move |_| bpm.write().tap(),
                    "Tap"
                }
                button { onclick: move |_| bpm.write().reset(), "Reset" }
            }
            if let Some(avg) = bpm.read().average() {
                if let Some(ms) = avg {
                    p {
                        span { style: "font-weight: bold", "{ms_to_bpm(ms)}" }
                        " ({ms}ms)"
                    }
                } else {
                    p { "First Tap" }
                }
            }
            ol {
                for ms in bpm.read().diff().take(18) {
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
