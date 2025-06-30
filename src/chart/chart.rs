use crate::prelude::*;

#[allow(clippy::absolute_paths, clippy::indexing_slicing)]
#[component]
pub fn Chart() -> Element {
    let state = use_context::<State>();
    let collection = state.entries.read();
    let entries = collection.entries.clone();
    let range = collection.range.clone();
    let weight_scatter: Vec<_> = entries
        .iter()
        .filter_map(|entry| {
            Some(ScatterData {
                point: Point::new(range.get_x(entry.date), range.get_y(entry.weight?)),
                class: get_shot_class(entry.shot.as_ref()),
                size: if entry.shot.is_some() { 0.0075 } else { 0.0050 },
                descender: entry
                    .weight_sma
                    .map(|descender| Point::new(range.get_x(entry.date), range.get_y(descender))),
            })
        })
        .collect();
    let sma_line: Vec<_> = entries
        .iter()
        .filter_map(|entry| {
            Some(Point::new(
                range.get_x(entry.date),
                range.get_y(entry.weight_sma?),
            ))
        })
        .collect();
    let smac_line: Vec<_> = entries
        .iter()
        .filter_map(|entry| {
            Some(Point::new(
                range.get_x(entry.date),
                range.get_y(entry.weight_sma_centered?),
            ))
        })
        .collect();
    rsx! {
        svg {
            view_box: "-0.1 -0.1 1.2 1.2",
            preserve_aspect_ratio: "xMidYMid slice",
            role: "img",
            Style {}
            ScatterChart {
                class: "sma",
                data: weight_scatter,
            }
            LineChart {
                class: "sma",
                points: sma_line,
            }
            LineChart {
                class: "smac",
                points: smac_line,
            }
        }
    }
}

fn get_shot_class(shot: Option<&Shot>) -> Option<String> {
    let shot = shot?;
    Some(format!("shot d{}", shot.dose * 10.0))
}
