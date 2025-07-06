use crate::prelude::*;

#[component]
pub fn Chart() -> Element {
    let state = use_context::<State>();
    let collection = state.entries.read().to_owned();
    let zoom = use_signal(|| Zoom::DEFAULT);
    let drag = use_signal(DragAction::default);
    let position = use_signal(ChartPosition::default);
    let factory = ChartFactory::new(collection);
    let weight_scatter: Vec<_> = factory.get_weight_scatter();
    let trend: Vec<Point> = factory.get_trend_points();
    let sma1c_line = factory.get_points_for(|x| x.sma1c);
    rsx! {
        svg {
            onwheel: move |event| Zoom::on_wheel(event, zoom),
            onmousedown: move |event| Drag::on_mousedown(event, drag),
            onmouseup: move |event| Drag::on_mouseup(event, drag, position),
            onmousemove: move |event| Drag::on_mousemove(event, drag, position),
            view_box: factory.get_viewbox(zoom, position),
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
