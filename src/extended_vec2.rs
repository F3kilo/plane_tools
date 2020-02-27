use glam::Vec2;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, Debug, Default)]
pub struct ExtendedVec2(Vec2);

impl PartialEq for ExtendedVec2 {
    fn eq(&self, other: &Self) -> bool {
        self.0.x() == other.0.x() && self.0.y() == other.0.y()
    }
}

impl Eq for ExtendedVec2 {}

impl PartialOrd for ExtendedVec2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let y_ord = self.0.y().partial_cmp(&other.0.y());
        if y_ord == Some(Ordering::Equal) {
            let x_ord = self.0.x().partial_cmp(&other.0.x());
            return x_ord;
        }
        y_ord
    }
}

impl Ord for ExtendedVec2 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).expect("can't compare some components are NaN")
    }
}

impl Hash<> for ExtendedVec2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.x().to_ne_bytes().hash(state);
        self.0.y().to_ne_bytes().hash(state);
    }
}