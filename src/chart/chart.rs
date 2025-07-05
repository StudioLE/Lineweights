use crate::prelude::*;

#[component]
pub fn Chart() -> Element {
    let state = use_context::<State>();
    let collection = state.entries.read().to_owned();
    let weight_scatter: Vec<_> = get_scatter(&collection);
    let trend: Vec<Point> = collection
        .get_trend(7, |x| x.sma1c)
        .unwrap_or_default()
        .into_iter()
        .map(|(day, value)| {
            Point::new(
                collection.range.x_from_day(day),
                collection.range.y_from_weight(value),
            )
        })
        .collect();
    let sma1c_line = get_points(&collection, |x| x.sma1c);
    let sma2_line = get_points(&collection, |x| x.sma2);
    let sma4_line = get_points(&collection, |x| x.sma4);
    let sma8_line = get_points(&collection, |x| x.sma8);
    rsx! {
        svg {
            view_box: "-0.1 -0.1 1.2 1.2",
            preserve_aspect_ratio: "xMidYMid slice",
            role: "img",
            Style {}
            DimensionChart {
                class: "trend",
                points: trend.clone(),
                y_scale: collection.range.y_scale,
            }
            LineChart {
                class: "trend",
                points: trend,
            }
            LineChart {
                class: "sma1c",
                points: sma1c_line,
            }
            LineChart {
                class: "sma2",
                points: sma2_line,
            }
            LineChart {
                class: "sma4",
                points: sma4_line,
            }
            LineChart {
                class: "sma8",
                points: sma8_line,
            }
            ScatterChart {
                class: None,
                data: weight_scatter,
            }
        }
    }
}

fn get_scatter(collection: &EntryCollection) -> Vec<ScatterData> {
    collection
        .entries
        .values()
        .filter_map(|entry| {
            let statistics = collection
                .statistics
                .get(&entry.date)
                .cloned()
                .unwrap_or_default();
            Some(ScatterData {
                point: Point::new(
                    collection.range.x_from_date(entry.date),
                    collection.range.y_from_weight(entry.weight?),
                ),
                class: get_shot_class(entry.shot.as_ref()),
                size: if entry.shot.is_some() { 0.0075 } else { 0.0050 },
                descender: statistics.sma1c.map(|descender| {
                    Point::new(
                        collection.range.x_from_date(entry.date),
                        collection.range.y_from_weight(descender),
                    )
                }),
            })
        })
        .collect()
}

fn get_points<F: Fn(&WeightStatistics) -> Option<f32>>(
    collection: &EntryCollection,
    predicate: F,
) -> Vec<Point> {
    collection
        .entries
        .values()
        .filter_map(|entry| {
            let statistics = collection
                .statistics
                .get(&entry.date)
                .cloned()
                .unwrap_or_default();
            Some(Point::new(
                collection.range.x_from_date(entry.date),
                collection.range.y_from_weight(predicate(&statistics)?),
            ))
        })
        .collect()
}

fn get_shot_class(shot: Option<&Shot>) -> Option<String> {
    let shot = shot?;
    Some(format!("shot d{}", shot.dose * 10.0))
}
