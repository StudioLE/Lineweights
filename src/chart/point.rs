#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn get_x(self) -> String {
        decimal_places(self.x)
    }

    pub fn get_y(self) -> String {
        decimal_places(self.y)
    }
}

pub fn decimal_places(value: f32) -> String {
    format!("{value:.6}")
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_owned()
}
