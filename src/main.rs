use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/style.css");
const SAKURA_CSS: &str = "https://cdn.jsdelivr.net/npm/sakura.css@1.5.1/css/sakura-earthly.css";

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: SAKURA_CSS }

        Main {}
    }
}

struct SNS<'a> {
    name: &'a str,
    href: &'a str,
    img: Asset,
}

#[component]
fn Main() -> Element {
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
    }
}
