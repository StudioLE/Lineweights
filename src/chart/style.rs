use crate::prelude::*;

#[component]
pub fn Style() -> Element {
    rsx! {
        style {
"
circle {{ fill: {SHOT_NONE} }}
.shot circle {{ fill: {SHOT_UNKNOWN} }}
.d25 circle {{ fill: {SHOT_25} }}
.d50 circle {{ fill: {SHOT_50} }}
.d75 circle {{ fill: {SHOT_75} }}
.d100 circle {{ fill: {SHOT_100} }}
.d125 circle {{ fill: {SHOT_125} }}
.d150 circle {{ fill: {SHOT_150} }}
line {{ 
  stroke-width: 0.0025;
  stroke: {SHOT_NONE}
}}
.shot line {{ stroke: {SHOT_UNKNOWN} }}
.d25 line {{ stroke: {SHOT_25} }}
.d50 line {{ stroke: {SHOT_50} }}
.d75 line {{ stroke: {SHOT_75} }}
.d100 line {{ stroke: {SHOT_100} }}
.d125 line {{ stroke: {SHOT_125} }}
.d150 line {{ stroke: {SHOT_150} }}
polyline {{
  fill: none;
  stroke: {SHOT_NONE};
  stroke-linejoin: bevel;
  stroke-width: 0.0025;
}}
polyline.sma1c {{
  stroke: {EMERALD_400};
  stroke-width: 0.0050;
}}
polyline.trend {{
  stroke: {EMERALD_800};
  stroke-width: 0.0025;
}}
"
        }
    }
}
