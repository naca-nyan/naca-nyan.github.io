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
            name: "github",
            href: "https://github.com/naca-nyan",
            img: asset!("/assets/github.svg"),
        },
        SNS {
            name: "bluesky",
            href: "https://bsky.app/profile/naca-nyan.bsky.social",
            img: asset!("/assets/bluesky.svg"),
        },
        SNS {
            name: "x",
            href: "https://x.com/naca_nyan",
            img: asset!("/assets/x.svg"),
        },
        SNS {
            name: "vrchat",
            href: "https://vrchat.com/home/user/usr_08cf7780-e093-4e29-a083-2bc7e4957247",
            img: asset!("/assets/vrchat.svg"),
        },
        SNS {
            name: "youtube",
            href: "https://www.youtube.com/@naca_nyan",
            img: asset!("/assets/youtube.svg"),
        },
    ];
    rsx! {
        TitleAndMeta { title: "Home", description: "なかにゃんのサイトだよ" }
        div { class: "mx-auto container px-3 typography",
            header { class: "my-10 flex flex-col items-center gap-5",
                div { class: "max-w-[500px] w-full",
                    img {
                        class: "rounded-full overflow-hidden",
                        src: asset!("/assets/icon.png"),
                    }
                }
                p { class: "font-extrabold text-3xl text-center", "なかにゃん @naca_nyan" }
                div { class: "flex gap-3",
                    for s in sns_list {
                        a { key: "{s.name}", href: s.href,
                            img { class: "rounded-sm w-16", src: s.img }
                        }
                    }
                }
            }
            h1 { class: "font-bold text-3xl", "便利なやつら" }
            hr { class: "my-3" }
            ul { class: "ms-6 list-disc text-xl",
                li {
                    Link { to: Route::Converter {}, "いろいろ変換するやつ" }
                }
                li {
                    Link { to: Route::BPM {}, "ぽちぽちしてBPM計測するやつ" }
                }
                li {
                    Link { to: Route::TimeGoal {}, "時間計算するやつ" }
                }
                li {
                    Link { to: Route::Password {}, "パスワード生成するやつ" }
                }
            }
        }
    }
}
