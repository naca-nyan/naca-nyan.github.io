use chrono::Timelike;
use dioxus::prelude::*;

use crate::TitleAndMeta;

fn now_minutes() -> i32 {
    let now = chrono::Local::now();
    let min = now.hour() * 60 + now.minute();
    min as i32
}

fn parse_as_minutes(s: &str) -> Result<i32, std::num::ParseIntError> {
    let n: i32 = s.trim().replace(':', "").parse()?;
    let v = (n / 100) * 60 + (n % 100);
    Ok(v)
}

fn fmt(v: i32) -> String {
    format!("{}:{:02}", v / 60, v % 60)
}

#[component]
pub fn TimeGoal() -> Element {
    let mut lines = use_signal(String::new);
    let mut goal_s = use_signal(|| String::from("08:00"));
    let mut now = use_signal::<Option<i32>>(|| None);
    let mut times: Vec<i32> = lines
        .read()
        .split("\n")
        .filter_map(|s| parse_as_minutes(s).ok())
        .collect();
    if times.len() % 2 == 1 {
        times.push(now_minutes());
    }
    let elapsed = times.chunks_exact(2).map(|v| v[1] - v[0]).sum();
    let goal = parse_as_minutes(goal_s.read().as_str()).unwrap_or_default();
    rsx! {
        TitleAndMeta {
            title: "時間計算するやつ",
            description: "時間計算するよ",
        }
        main { class: "typography ui",
            h1 { class: "text-xl font-semibold", "目標まであと何分？" }
            hr { class: "my-3" }
            div { class: "my-2 flex items-center gap-3",
                "目標"
                input {
                    class: "p-1",
                    r#type: "time",
                    value: "{goal_s}",
                    oninput: move |e| *goal_s.write() = e.value(),
                }
            }
            textarea {
                class: "block p-2 w-full",
                value: "{lines}",
                rows: 4,
                oninput: move |e| {
                    *lines.write() = e.value();
                    *now.write() = (!e.value().is_empty()).then(now_minutes);
                },
            }
            div { class: "mt-3",
                if let Some(n) = now() {
                    p { "いま: {fmt(n)}" }
                    p { "経過時間: {fmt(elapsed)}" }
                    if goal > elapsed {
                        p { "達成予定時刻: {fmt(n + goal - elapsed)}" }
                        if times.len() == 2 {
                            p { "休憩込み: {fmt(n + goal - elapsed + 60)}" }
                        }
                    } else {
                        p { "目標達成！" }
                    }
                }
            }
        }
    }
}
