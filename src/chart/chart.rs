use crate::prelude::*;

#[component]
pub fn Chart() -> Element {
    let state = use_context::<State>();
    let collection = state.entries.read().to_owned();
    let factory = ChartFactory::new(collection);
    let weight_scatter: Vec<_> = factory.get_weight_scatter();
    let trend: Vec<Point> = factory.get_trend_points();
    let sma1c_line = factory.get_points_for(|x| x.sma1c);
    rsx! {
        svg {
            view_box: "-0.1 -0.1 1.2 1.2",
            preserve_aspect_ratio: "xMidYMid slice",
            role: "img",
            Style {}
            DimensionChart {
                class: "trend",
                points: trend.clone(),
                y_scale: factory.y_scale,
            }
            LineChart {
                class: "trend",
                points: trend,
            }
            LineChart {
                class: "sma1c",
                points: sma1c_line,
            }
            ScatterChart {
                class: None,
                data: weight_scatter,
            }
        }
    }
}
