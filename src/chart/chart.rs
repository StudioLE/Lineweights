use crate::prelude::*;

#[component]
pub fn Chart() -> Element {
    let state = use_context::<State>();
    let collection = state.entries.read();
    let entries = collection.entries.clone();
    let range = collection.range.clone();
    let weight_scatter: Vec<_> = get_scatter(&entries, &range);
    let sma1c_line = get_points(&entries, &range, |x| x.sma1c);
    let sma2_line = get_points(&entries, &range, |x| x.sma2);
    let sma4_line = get_points(&entries, &range, |x| x.sma4);
    let sma8_line = get_points(&entries, &range, |x| x.sma8);
    rsx! {
        svg {
            view_box: "-0.1 -0.1 1.2 1.2",
            preserve_aspect_ratio: "xMidYMid slice",
            role: "img",
            Style {}
            LineChart {
                class: "sma1c",
                points: sma1c_line,
            }
            LineChart {
                class: "sma",
                points: sma2_line,
            }
            LineChart {
                class: "sma",
                points: sma4_line,
            }
            LineChart {
                class: "sma",
                points: sma8_line,
            }
            ScatterChart {
                class: None,
                data: weight_scatter,
            }
        }
    }
}

fn get_scatter(entries: &Vec<Entry>, range: &EntryRange) -> Vec<ScatterData> {
    entries
        .iter()
        .filter_map(|entry| {
            Some(ScatterData {
                point: Point::new(range.get_x(entry.date), range.get_y(entry.weight?)),
                class: get_shot_class(entry.shot.as_ref()),
                size: if entry.shot.is_some() { 0.0075 } else { 0.0050 },
                descender: entry
                    .statistics
                    .sma1c
                    .map(|descender| Point::new(range.get_x(entry.date), range.get_y(descender))),
            })
        })
        .collect()
}

fn get_points<F: Fn(&WeightStatistics) -> Option<f32>>(
    entries: &Vec<Entry>,
    range: &EntryRange,
    predicate: F,
) -> Vec<Point> {
    entries
        .iter()
        .filter_map(|entry| {
            Some(Point::new(
                range.get_x(entry.date),
                range.get_y(predicate(&entry.statistics)?),
            ))
        })
        .collect()
}

fn get_shot_class(shot: Option<&Shot>) -> Option<String> {
    let shot = shot?;
    Some(format!("shot d{}", shot.dose * 10.0))
}
