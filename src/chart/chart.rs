use crate::prelude::*;

#[allow(clippy::absolute_paths, clippy::indexing_slicing)]
#[component]
pub fn Chart() -> Element {
    let state = use_context::<State>();
    let mut entries = state.entries.read().clone();
    let range = EntryRange::new(&entries)?;
    Processor::execute(&mut entries, &range);
    rsx! {
        svg {
            view_box: "-0.1 -0.1 1.2 1.2",
            preserve_aspect_ratio: "xMidYMid slice",
            role: "img",
            style {
"
circle.sma {{ visibility: hidden }}
circle {{ fill: {SHOT_NONE} }}
circle.shot {{ fill: {SHOT_UNKNOWN} }}
circle.d25 {{ fill: {SHOT_25} }}
circle.d50 {{ fill: {SHOT_50} }}
circle.d75 {{ fill: {SHOT_75} }}
circle.d100 {{ fill: {SHOT_100} }}
circle.d125 {{ fill: {SHOT_125} }}
circle.d150 {{ fill: {SHOT_150} }}
line {{ stroke-width: 0.0025 }}
line {{ stroke: {SHOT_NONE} }}
line.sma {{ stroke-width: 0.0025 }}
line.sma {{ stroke: {EMERALD_900} }}
line.smac {{ stroke-width: 0.0050 }}
line.smac {{ stroke: {EMERALD_400} }}
line.shot {{ stroke: {SHOT_UNKNOWN} }}
line.d25 {{ stroke: {SHOT_25} }}
line.d50 {{ stroke: {SHOT_50} }}
line.d75 {{ stroke: {SHOT_75} }}
line.d100 {{ stroke: {SHOT_100} }}
line.d125 {{ stroke: {SHOT_125} }}
line.d150 {{ stroke: {SHOT_150} }}
"
            }
            for entry in entries.iter() {
                if let Some(weight_sma) = entry.weight_sma {
                    if let  Some(weight) = entry.weight {
                        line {
                            class: get_shot_class(entry.shot.as_ref()),
                            x1: range.get_x(entry.date),
                            y1: range.get_y(weight),
                            x2: range.get_x(entry.date),
                            y2: range.get_y(weight_sma),
                        }
                    }
                }
                if let Some(weight) = entry.weight {
                    circle {
                        class: get_shot_class(entry.shot.as_ref()),
                        cx: range.get_x(entry.date),
                        cy: range.get_y(weight),
                        r: if entry.shot.is_some() { 0.0075 } else { 0.0050 },
                    }
                }
            }
            for pair in entries.windows(2) {
                if let Some(left) = pair[0].weight_sma {
                    if let Some(right) = pair[1].weight_sma {
                        line {
                            class: "sma",
                            x1: range.get_x(pair[0].date),
                            y1: range.get_y(left),
                            x2: range.get_x(pair[1].date),
                            y2: range.get_y(right),
                        }
                    }
                }
                if let Some(left) = pair[0].weight_sma_centered {
                    if let Some(right) = pair[1].weight_sma_centered {
                        line {
                            class: "smac",
                            x1: range.get_x(pair[0].date),
                            y1: range.get_y(left),
                            x2: range.get_x(pair[1].date),
                            y2: range.get_y(right),
                        }
                    }
                }
            }
            for entry in entries.iter() {
                if let Some(weight_sma) = entry.weight_sma {
                    circle {
                        class: format!("sma {}", get_shot_class(entry.shot.as_ref())),
                        cx: range.get_x(entry.date),
                        cy: range.get_y(weight_sma),
                        r: if entry.shot.is_some() { 0.0075 } else { 0.0050 },
                    }
                }
            }
        }
    }
}

fn get_shot_class(shot: Option<&Shot>) -> String {
    if let Some(shot) = shot {
        format!("shot d{}", shot.dose * 10.0)
    } else {
        String::new()
    }
}
