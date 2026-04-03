use dioxus::prelude::*;

use crate::TitleAndMeta;

use rand::RngExt;

fn new_password(charset: &[u8], len: i32) -> String {
    (0..len)
        .map(|_| {
            let idx = rand::rng().random_range(0..charset.len());
            charset[idx] as char
        })
        .collect()
}

fn new_passwords(charset: &[u8], len: i32, count: usize) -> Vec<String> {
    (0..count)
        .into_iter()
        .map(|_| new_password(charset, len))
        .collect()
}

#[component]
pub fn Password() -> Element {
    let mut charset = use_signal(|| {
        String::from(
            r###"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()_-+=\|[]{}:;"',.<>/?~`"###,
        )
    });
    let mut passwords = use_signal(Vec::<String>::new);
    let mut len = use_signal(|| String::from("12"));

    rsx! {
        TitleAndMeta {
            title: "パスワード作るやつ",
            description: "ランダムなパスワードをクライアント側で作るよ",
        }
        h1 { "パスワード作るやつ" }
        div {
            "文字種類"
            input {
                name: "characters to use",
                style: "width: 100%; font-family: monospace",
                r#type: "text",
                value: charset,
                oninput: move |e| { *charset.write() = e.value() },
            }
            "長さ"
            input {
                r#type: "number",
                value: len,
                oninput: move |e| { *len.write() = e.value() },
            }
        }
        button {
            onclick: move |_| {
                *passwords.write() = new_passwords(
                    charset.read().as_bytes(),
                    len.read().parse().unwrap_or_default(),
                    60,
                )
            },
            "作る"
        }
        div {
            pre { style: "display: flex; flex-wrap: wrap; justify-content: center; gap: 30px",
                for p in passwords.read().clone() {
                    code { "{p}" }
                }
            }
        }
    }
}
