use glam::Vec2;
use std::cmp::Ordering;

/// Compare vectors by next rules:
/// 1) if `l.y > r.y` => `l > r`
/// 2) if `l.y == r.y` and `l.x > r.x` => `l < r`
/// 3) else `l == r`
/// # Panics
/// Panics if some coordinates are NaN
/// # Examples
/// ```
/// use plane_tools::vec2cmp::cmp_y;
/// use glam::Vec2;
/// use std::cmp::Ordering;
///
/// let a = Vec2::new(1f32, 0f32);
/// let b = Vec2::new(0f32, 1f32);
/// assert_eq!(cmp_y(a, b), Ordering::Less);
/// ```
pub fn cmp_y(l: Vec2, r: Vec2) -> Ordering {
    let y_ord = l
        .y()
        .partial_cmp(&r.y())
        .expect("Can't compare vectors. Y component is NaN!");
    if y_ord == Ordering::Equal {
        let x_ord = l
            .x()
            .partial_cmp(&r.x())
            .expect("Can't compare vectors. X component is NaN!");
        return x_ord.reverse();
    }
    y_ord
}

#[cfg(test)]
mod tests {
    use super::cmp_y;
    use glam::Vec2;
    use std::cmp::Ordering;

    fn default_vecs() -> (Vec2, Vec2, Vec2, Vec2) {
        (
            Vec2::new(0f32, 0f32),
            Vec2::new(1f32, 0f32),
            Vec2::new(0f32, 1f32),
            Vec2::new(1f32, 0f32),
        )
    }

    #[test]
    fn cmp_y_test() {
        let (a, b, c, d) = default_vecs();
        assert_eq!(cmp_y(a, b), Ordering::Greater);
        assert_eq!(cmp_y(a, c), Ordering::Less);
        assert_eq!(cmp_y(b, d), Ordering::Equal);
        assert_eq!(cmp_y(c, a), Ordering::Greater);
    }

    #[test]
    #[should_panic]
    fn cmp_y_nan() {
        let nan_vec = Vec2::new(0f32 / 0f32, 0f32);
        let (a, _, _, _) = default_vecs();
        cmp_y(a, nan_vec);
    }
}
