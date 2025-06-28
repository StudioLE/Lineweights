use crate::prelude::*;

#[allow(clippy::absolute_paths)]
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
            for entry in entries.iter() {
                if let Some(weight_sma) = entry.weight_sma {
                    if let  Some(weight) = entry.weight {
                        line {
                            x1: range.get_x(entry.date),
                            y1: range.get_y(weight),
                            x2: range.get_x(entry.date),
                            y2: range.get_y(weight_sma),
                            stroke: Shot::get_color(&entry.shot),
                            stroke_width: 0.0025,
                        }
                    }
                }
                if let Some(weight) = entry.weight {
                    circle {
                        cx: range.get_x(entry.date),
                        cy: range.get_y(weight),
                        r: if entry.shot.is_some() { 0.0075 } else { 0.0050 },
                        fill: Shot::get_color(&entry.shot)
                    }
                }
            }
            for pair in entries.windows(2) {
                if let Some(left) = pair[0].weight_sma {
                    if let Some(right) = pair[1].weight_sma {
                        line {
                            x1: range.get_x(pair[0].date),
                            y1: range.get_y(left),
                            x2: range.get_x(pair[1].date),
                            y2: range.get_y(right),
                            stroke: EMERALD_400,
                            stroke_width: 0.0050,
                        }
                    }
                }
            }
            for entry in entries.iter() {
                if let Some(weight_sma) = entry.weight_sma {
                    circle {
                        cx: range.get_x(entry.date),
                        cy: range.get_y(weight_sma),
                        r: if entry.shot.is_some() { 0.0075 } else { 0.0050 },
                        visibility: "hidden",
                        fill: Shot::get_color(&entry.shot)
                    }
                }
            }
        }
    }
}
