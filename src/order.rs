use glam::Vec2;
use std::cmp::Ordering;

pub fn desc_ord(vecs: &[Vec2], cmp: impl Fn(Vec2, Vec2) -> Ordering) -> Vec<usize> {
    let vecs_count = vecs.len();
    let mut indices: Vec<usize> = (0..vecs_count).collect();
    indices.sort_unstable_by(|l, r| cmp(vecs[*l], vecs[*r]).reverse());
    indices
}

pub fn asc_ord(vecs: &[Vec2], cmp: impl Fn(Vec2, Vec2) -> Ordering) -> Vec<usize> {
    let vecs_count = vecs.len();
    let mut indices: Vec<usize> = (0..vecs_count).collect();
    indices.sort_unstable_by(|l, r| cmp(vecs[*l], vecs[*r]));
    indices
}

#[cfg(test)]
mod tests {
    use super::{asc_ord, desc_ord};
    use crate::vec2cmp::cmp_y;
    use glam::Vec2;

    #[test]
    fn desc_ord_test() {
        let vecs = vec![
            Vec2::new(0f32, -2f32),
            Vec2::new(0f32, 3f32),
            Vec2::new(1f32, 1f32),
            Vec2::new(0f32, 1f32),
        ];

        let order = desc_ord(&vecs, cmp_y);
        assert_eq!(order[0], 1);
        assert_eq!(order[1], 3);
        assert_eq!(order[2], 2);
        assert_eq!(order[3], 0);
    }

    #[test]
    fn asc_ord_test() {
        let vecs = vec![
            Vec2::new(0f32, -2f32),
            Vec2::new(0f32, 3f32),
            Vec2::new(1f32, 1f32),
            Vec2::new(0f32, 1f32),
        ];

        let order = asc_ord(&vecs, cmp_y);
        assert_eq!(order[0], 0);
        assert_eq!(order[1], 2);
        assert_eq!(order[2], 3);
        assert_eq!(order[3], 1);
    }
}
