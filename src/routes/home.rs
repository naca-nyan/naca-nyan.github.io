use dioxus::prelude::*;

use crate::{Route, TitleAndMeta};

struct SNS<'a> {
    name: &'a str,
    href: &'a str,
    img: Asset,
}

#[component]
pub fn Home() -> Element {
    let sns_list = [
        SNS {
            name: "x",
            href: "https://x.com/naca_nyan",
            img: asset!("/assets/x.svg"),
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
