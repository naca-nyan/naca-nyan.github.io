use chrono::Timelike;
use dioxus::prelude::*;

use crate::TitleAndMeta;

fn now() -> i32 {
    let now = chrono::Local::now();
    let min = now.hour() * 60 + now.minute();
    min as i32
}

fn try_from(value: &str) -> Result<i32, std::num::ParseIntError> {
    let n: i32 = value.replace(':', "").parse()?;
    let v = (n / 100) * 60 + (n % 100);
    Ok(v)
}

fn fmt(v: i32) -> String {
    format!("{}:{:02}", v / 60, v % 60)
}

#[component]
pub fn TimeGoal() -> Element {
    let mut s = use_signal(String::new);
    let mut goal_s = use_signal(|| String::from("8:00"));
    let mut now_signal = use_signal::<Option<i32>>(|| None);
    let mut times: Vec<i32> = s
        .read()
        .split("\n")
        .filter_map(|s| try_from(s).ok())
        .collect();
    if times.len() % 2 == 1 {
        times.push(now());
    }
    let elapsed = times.chunks_exact(2).map(|v| v[1] - v[0]).sum();
    let goal = try_from(goal_s.read().as_str()).unwrap_or_default();
    rsx! {
        TitleAndMeta {
            title: "時間計算するやつ",
            description: "時間計算するよ",
        }
        h1 { "目標まであと何分？" }
        "目標"
        input {
            r#type: "text",
            value: "{goal_s}",
            oninput: move |e| *goal_s.write() = e.value(),
        }
        textarea {
            value: "{s}",
            rows: 4,
            oninput: move |e| {
                *s.write() = e.value();
                *now_signal.write() = (!e.value().is_empty()).then(now);
            },
        }
        if let Some(n) = now_signal() {
            div {
                "いま: {fmt(n)}"
                br {}
                "経過時間: {fmt(elapsed)}"
                br {}
                if goal > elapsed {
                    "達成予定時刻: {fmt(n + goal - elapsed)}"
                } else {
                    "目標達成！"
                }
            }
        } else {
            "なにか入力してね"
        }
    }
}
