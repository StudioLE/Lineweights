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
            for entry in entries {
                if let Some(weight) = entry.weight {
                    circle {
                        cx: range.get_x(entry.date),
                        cy: range.get_y(weight),
                        r: 0.0025,
                        fill: "#1f2937"
                    }
                }
                if let Some(weight_sma) = entry.weight_sma {
                    circle {
                        cx: range.get_x(entry.date),
                        cy: range.get_y(weight_sma),
                        r: if entry.shot.is_some() { 0.0075 } else { 0.005 },
                        fill: get_color(entry.shot)
                    }
                }
            }
        }
    }
}

fn get_color(shot: Option<Shot>) -> String {
    let color = if let Some(shot) = shot {
        if shot.dose == 2.5 {
            "#525252"
        } else if shot.dose == 5.0 {
            "#581c87"
        } else if shot.dose == 7.5 {
            "#0f766e"
        } else if shot.dose == 10.0 {
            "#db2777"
        } else if shot.dose == 12.5 {
            "#0ea5e9"
        } else if shot.dose == 15.0 {
            "#f87171"
        } else {
            "#64748b"
        }
    } else {
        "#374151"
    };
    color.to_owned()
}