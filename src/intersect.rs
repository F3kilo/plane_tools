use glam::Vec2;

/// Detect intersections (or the lack of it) of two line segments
pub fn intersect_point(lseg1: (Vec2, Vec2), lseg2: (Vec2, Vec2)) -> Option<Vec2> {
    let (oa1, oa2) = (lseg1.0, lseg1.1);
    let (ob1, ob2) = (lseg2.0, lseg2.1);

    let a = oa2 - oa1;
    let b = ob2 - ob1;
    let c = oa1 - ob1;

    let num = c.x() * b.y() - b.x() * c.y();
    let den = b.x() * a.y() - a.x() * b.y();

    let alpha = num / den;
    println!("alpha: {}", alpha);
    if alpha < 0f32 || alpha > 1f32 {
        return None;
    }

    let gamma = (c.x() + alpha * a.x()) / b.x();
    println!("gamma: {}", alpha);
    if gamma < 0f32 || gamma > 1f32 {
        return None;
    }

    if !alpha.is_nan() {
        let x = oa1 + a * alpha;
        return Some(x);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::intersect_point;
    use glam::Vec2;

    struct TestData {
        l1: (Vec2, Vec2),
        l2: (Vec2, Vec2),
        res: Option<Vec2>,
    }

    fn basic_intersect() -> TestData {
        let a1 = (2f32, 2f32).into();
        let a2 = (8f32, 8f32).into();
        let b1 = (8f32, 2f32).into();
        let b2 = (2f32, 8f32).into();
        TestData {
            l1: (a1, a2),
            l2: (b1, b2),
            res: Some((5f32, 5f32).into()),
        }
    }

    fn intersect_after_a2() -> TestData {
        let a1 = (2f32, 2f32).into();
        let a2 = (8f32, 8f32).into();
        let b1 = (28f32, 22f32).into();
        let b2 = (22f32, 28f32).into();
        TestData {
            l1: (a1, a2),
            l2: (b1, b2),
            res: None,
        }
    }

    fn intersect_before_a1() -> TestData {
        let a1 = (2f32, 2f32).into();
        let a2 = (8f32, 8f32).into();
        let b1 = (-8f32, -2f32).into();
        let b2 = (-2f32, -8f32).into();
        TestData {
            l1: (a1, a2),
            l2: (b1, b2),
            res: None,
        }
    }

    fn intersect_after_b2() -> TestData {
        let a1 = (-8f32, 12f32).into();
        let a2 = (-2f32, 18f32).into();
        let b1 = (8f32, 2f32).into();
        let b2 = (2f32, 8f32).into();
        TestData {
            l1: (a1, a2),
            l2: (b1, b2),
            res: None,
        }
    }

    fn intersect_before_b1() -> TestData {
        let a1 = (12f32, -8f32).into();
        let a2 = (18f32, -2f32).into();
        let b1 = (8f32, 2f32).into();
        let b2 = (2f32, 8f32).into();
        TestData {
            l1: (a1, a2),
            l2: (b1, b2),
            res: None,
        }
    }

    fn intersect_in_b2() -> TestData {
        let a1 = (0f32, 6f32).into();
        let a2 = (6f32, 12f32).into();
        let b1 = (8f32, 2f32).into();
        let b2 = (2f32, 8f32).into();
        TestData {
            l1: (a1, a2),
            l2: (b1, b2),
            res: Some((2f32, 8f32).into()),
        }
    }

    fn intersect_in_b1() -> TestData {
        let a1 = (4f32, -2f32).into();
        let a2 = (10f32, 4f32).into();
        let b1 = (8f32, 2f32).into();
        let b2 = (2f32, 8f32).into();
        TestData {
            l1: (a1, a2),
            l2: (b1, b2),
            res: None,
        }
    }

    fn intersect_in_a2() -> TestData {
        let a1 = (8f32, 2f32).into();
        let a2 = (2f32, 8f32).into();
        let b1 = (0f32, 6f32).into();
        let b2 = (6f32, 12f32).into();
        TestData {
            l1: (a1, a2),
            l2: (b1, b2),
            res: Some((2f32, 8f32).into()),
        }
    }

    fn intersect_in_a1() -> TestData {
        let a1 = (8f32, 2f32).into();
        let a2 = (2f32, 8f32).into();
        let b1 = (4f32, -2f32).into();
        let b2 = (10f32, 4f32).into();
        TestData {
            l1: (a1, a2),
            l2: (b1, b2),
            res: None,
        }
    }

    #[test]
    fn intersect_point_test() {
        let data = basic_intersect();
        assert_eq!(intersect_point(data.l1, data.l2), data.res);

        let data = intersect_after_a2();
        assert_eq!(intersect_point(data.l1, data.l2), data.res);

        let data = intersect_before_a1();
        assert_eq!(intersect_point(data.l1, data.l2), data.res);

        let data = intersect_after_b2();
        assert_eq!(intersect_point(data.l1, data.l2), data.res);

        let data = intersect_before_b1();
        assert_eq!(intersect_point(data.l1, data.l2), data.res);

        let data = intersect_in_b2();
        assert_eq!(intersect_point(data.l1, data.l2), data.res);

        let data = intersect_in_b1();
        assert_eq!(intersect_point(data.l1, data.l2), data.res);

        let data = intersect_in_a2();
        assert_eq!(intersect_point(data.l1, data.l2), data.res);

        let data = intersect_in_a1();
        assert_eq!(intersect_point(data.l1, data.l2), data.res);
    }
}
