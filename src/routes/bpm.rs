use dioxus::prelude::*;

use crate::TitleAndMeta;

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
pub fn BPM() -> Element {
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
