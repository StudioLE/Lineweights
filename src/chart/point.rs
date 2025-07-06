use crate::prelude::dioxus_elements::geometry::PagePoint;
use crate::prelude::*;
use std::ops::{Add, Sub};

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
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

impl Display for Point {
    #[allow(clippy::absolute_paths)]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl From<PagePoint> for Point {
    #[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
    fn from(page: PagePoint) -> Self {
        Self::new(page.x as f32, page.y as f32)
    }
}

pub fn decimal_places(value: f32) -> String {
    format!("{value:.6}")
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_owned()
}
