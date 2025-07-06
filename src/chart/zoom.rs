use crate::prelude::*;

pub struct Zoom;

impl Zoom {
    pub(crate) const DEFAULT: usize = Self::MIN;
    const MIN: usize = 1;
    const MAX: usize = 100;

    pub(crate) fn on_wheel(event: Event<WheelData>, zoom: Signal<usize>) {
        let delta = event.delta().strip_units().y;
        if delta < 0.0 {
            zoom_in(zoom);
        } else if delta > 0.0 {
            zoom_out(zoom);
        } else {
            debug!("No delta");
        }
    }

    #[allow(clippy::as_conversions, clippy::cast_precision_loss)]
    pub(crate) fn get_scale(signal: Signal<usize>) -> f32 {
        let zoom = *signal.read() as f32;
        let base: f32 = 1.0;
        let growth_factor: f32 = 0.9;
        let scale = base * growth_factor.powf(zoom - 1.0);
        debug!("Chart zoom: {zoom}");
        debug!("Chart scale: {scale:.2}");
        scale
    }
}

fn zoom_in(mut signal: Signal<usize>) {
    let value = *signal.read();
    if value < Zoom::MAX {
        debug!("Zooming in to {}", value + 1);
        signal.set(signal + 1);
    } else {
        debug!("Reached max zoom level: {value}");
    }
}

fn zoom_out(mut signal: Signal<usize>) {
    let value = *signal.read();
    if value > Zoom::MIN {
        debug!("Zooming out to {}", value - 1);
        signal.set(value - 1);
    } else {
        debug!("Reached min zoom level: {value}");
    }
}
