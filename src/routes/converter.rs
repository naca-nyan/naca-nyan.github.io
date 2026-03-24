use dioxus::prelude::*;

use crate::TitleAndMeta;

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
pub fn Converter() -> Element {
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
