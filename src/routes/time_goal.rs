use chrono::Timelike;
use dioxus::prelude::*;

use crate::TitleAndMeta;

#[derive(Clone, Copy)]
struct HourMin(i32);

impl HourMin {
    fn now() -> HourMin {
        let now = chrono::Local::now();
        let min = now.hour() * 60 + now.minute();
        HourMin(min as i32)
    }
}

impl TryFrom<&str> for HourMin {
    type Error = std::num::ParseIntError;
    fn try_from(value: &str) -> Result<HourMin, Self::Error> {
        let n: i32 = value.replace(':', "").parse()?;
        let v = (n / 100) * 60 + (n % 100);
        Ok(HourMin(v))
    }
}

impl std::ops::Add for HourMin {
    type Output = HourMin;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::AddAssign for HourMin {
    fn add_assign(&mut self, rhs: Self) {
        self.0 = self.0 + rhs.0
    }
}

impl std::ops::Sub for HourMin {
    type Output = HourMin;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl std::fmt::Display for HourMin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.0 / 60, self.0 % 60)
    }
}

#[component]
pub fn TimeGoal() -> Element {
    let mut s = use_signal(String::new);
    let mut goal = use_signal(|| String::from("8:00"));
    let times: Vec<HourMin> = s
        .read()
        .split("\n")
        .filter_map(|s| HourMin::try_from(s).ok())
        .collect();
    let mut iter = times.chunks_exact(2);
    let mut elapsed = HourMin(0);
    while let Some(v) = iter.next() {
        elapsed += v[1] - v[0]
    }
    let v = iter.remainder();
    if v.len() > 0 {
        elapsed += HourMin::now() - v[0]
    }
    let now = use_signal(HourMin::now);
    let remaining = HourMin::try_from(goal.read().as_str())
        .map(|v| v.0)
        .unwrap_or_default()
        - elapsed.0;
    rsx! {
        TitleAndMeta {
            title: "時間計算するやつ",
            description: "時間計算するよ",
        }
        h1 { "目標まであと何分？" }
        "目標"
        input {
            r#type: "text",
            value: "{goal}",
            oninput: move |e| *goal.write() = e.value(),
        }
        textarea { value: "{s}", rows: 4, oninput: move |e| *s.write() = e.value() }
        div {
            "いま: {now}"
            br {}
            "経過時間: {elapsed}"
            br {}
            if remaining > 0 {
                "達成予定時刻: {HourMin::now() + HourMin(remaining)}"
            } else {
                "目標達成！"
            }
        }
    }
}
