use crate::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct Drag;
#[derive(Clone, Debug, Default)]
pub struct ChartPosition {
    pub value: Point,
}

#[derive(Clone, Debug, Default)]
pub struct DragAction {
    pub from: Option<Point>,
}

impl Drag {
    pub fn on_mousedown(event: Event<MouseData>, mut action: Signal<DragAction>) {
        event.prevent_default();
        let page = event.data.page_coordinates();
        action.set(DragAction {
            from: Some(page.into()),
        });
    }

    pub fn on_mousemove(
        event: Event<MouseData>,
        mut action: Signal<DragAction>,
        mut position: Signal<ChartPosition>,
    ) {
        event.prevent_default();
        update_position(event, &mut action, &mut position);
    }

    pub fn on_mouseup(
        event: Event<MouseData>,
        mut action: Signal<DragAction>,
        mut position: Signal<ChartPosition>,
    ) {
        event.prevent_default();
        update_position(event, &mut action, &mut position);
        action.set(DragAction { from: None });
    }
}

fn update_position(
    event: Event<MouseData>,
    action: &mut Signal<DragAction>,
    position: &mut Signal<ChartPosition>,
) {
    let Some(from) = action.read().from else {
        return;
    };
    let current: Point = event.data.page_coordinates().into();
    action.set(DragAction {
        from: Some(current),
    });
    let delta = from - current;
    let current = position.read().value;
    let value = delta + position.read().value;
    debug!("Moving {delta} from {current} to {value}");
    position.set(ChartPosition { value });
}
