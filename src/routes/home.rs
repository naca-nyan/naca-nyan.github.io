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
        div { class: "typography",
            header { class: "px-8 flex flex-col items-center gap-5",
                div { class: "max-w-[400px] w-full",
                    img {
                        class: "rounded-full overflow-hidden",
                        src: asset!("/assets/icon.webp"),
                    }
                }
                p { class: "font-extrabold text-2xl text-center", "なかにゃん @naca_nyan" }
                div { class: "flex gap-3",
                    for s in sns_list {
                        a { key: "{s.name}", href: s.href,
                            img { class: "rounded-sm w-12", src: s.img }
                        }
                    }
                }
            }
            h1 { class: "mt-3 font-bold text-xl", "つくったもの" }
            hr { class: "my-3" }
            ul { class: "ms-6 list-disc",
                li {
                    a { href: "https://prali.net", "prali.net" }
                    p { "VRアイドルバンドPRA-LiNÉ公式サイト" }
                }
                li {
                    a { href: "https://github.com/naca-nyan/vst-viseme", "vst-viseme" }
                    p { "声の高さとかをVRCにOSC送信できるVSTPlugin" }
                }
                li {
                    a { href: "https://syncroom-repertoire.web.app", "syncroom-repertoire.web.app" }
                    p { "知ってる曲を登録できるウェブアプリ" }
                }
                li {
                    a { href: "https://chromewebstore.google.com/detail/chordwiki-bold/jediokeakjffilhbalfckcncgoojmioe",
                        "ChordWiki Bold"
                    }
                    p { "Chordwiki を見やすくするブラウザ拡張 (awawainu と共作)" }
                }
            }
            h1 { class: "mt-3 font-bold text-xl", "便利なやつら" }
            hr { class: "my-3" }
            ul { class: "ms-6 list-disc",
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
