use crate::vec2cmp;
use glam::Vec2;
use std::cmp::Ordering;

pub struct HeightOrdVec2(pub Vec2);

// Newtype pattern for glam::Vec2 to allow compare two vectors by height.
impl From<Vec2> for HeightOrdVec2 {
    fn from(v: Vec2) -> Self {
        Self(v)
    }
}

impl PartialEq for HeightOrdVec2 {
    fn eq(&self, r: &Self) -> bool {
        vec2cmp::cmp_y(self.0, r.0) == Ordering::Equal
    }
}

impl PartialOrd for HeightOrdVec2 {
    fn partial_cmp(&self, r: &Self) -> Option<Ordering> {
        Some(vec2cmp::cmp_y(self.0, r.0))
    }
}

impl Eq for HeightOrdVec2 {}

impl Ord for HeightOrdVec2 {
    fn cmp(&self, r: &Self) -> std::cmp::Ordering {
        self.partial_cmp(r).unwrap()
    }
}
