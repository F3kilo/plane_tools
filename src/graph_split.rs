use crate::graph::Graph;
use crate::height_ord_vec2::HeightOrdVec2;
use crate::intersect::intersect_point;
use crate::vec2cmp;
use glam::Vec2;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::iter::FromIterator;

pub fn into_no_intersect(mut g: Graph<Vec2>) -> Graph<Vec2> {
    let mut heap = vertices_to_heap(g.vertices());
    let mut edges: HashSet<(usize, usize)> = HashSet::new();
    while let Some(p) = heap.pop() {
        let p = p.0;
        let s = unchecked_edges(&g, p, &mut edges);
    }
    Graph::new()
}

struct IntersectionPoint {
    q: usize,
    edge: (usize, usize),
    i: Vec2,
}

fn highest_intersection_point(
    g: &Graph<Vec2>,
    p: usize,
    s: &HashSet<usize>,
    edges: &Vec<(usize, usize)>,
) -> Option<IntersectionPoint> {
    let mut top_intersection_point = None;
    for q in s {
        for (v1, v2) in edges {
            let i = intersect_point((g[p], g[*q]), (g[*v1], g[*v2]));
            try_make_higher(&mut top_intersection_point, i, *q, (*v1, *v2));
        }
    }
    top_intersection_point
}

fn try_make_higher(
    i: &mut Option<IntersectionPoint>,
    pos: Option<Vec2>,
    q: usize,
    edge: (usize, usize),
) {
    if let Some(pos) = pos {
        if let Some(ip) = i {
            if vec2cmp::cmp_y(ip.i, pos) == Ordering::Greater {
                *i = Some(IntersectionPoint { q, edge, i: pos });
            }
        } else {
            *i = Some(IntersectionPoint { q, edge, i: pos });
        }
    }
}

fn unchecked_edges(
    g: &Graph<Vec2>,
    p: usize,
    edges: &mut HashSet<(usize, usize)>,
) -> HashSet<usize> {
    let mut s = g.connects_of(p).clone();
    for q in s.clone() {
        if edges.remove(&(q, p)) {
            s.remove(&q);
        }
    }
    s
}

struct HeightOrdGraphVert(usize, HeightOrdVec2);

impl HeightOrdGraphVert {
    fn new(index: usize, vert: Vec2) -> Self {
        Self(index, vert.into())
    }
}

impl PartialEq for HeightOrdGraphVert {
    fn eq(&self, r: &Self) -> bool {
        self.1.cmp(&r.1) == Ordering::Equal
    }
}

impl PartialOrd for HeightOrdGraphVert {
    fn partial_cmp(&self, r: &Self) -> Option<Ordering> {
        self.1.partial_cmp(&r.1)
    }
}

impl Eq for HeightOrdGraphVert {}

impl Ord for HeightOrdGraphVert {
    fn cmp(&self, r: &Self) -> std::cmp::Ordering {
        self.1.cmp(&r.1)
    }
}

fn vertices_to_heap(verts: &Vec<Vec2>) -> BinaryHeap<HeightOrdGraphVert> {
    BinaryHeap::from_iter(
        verts
            .iter()
            .enumerate()
            .map(|(i, v)| HeightOrdGraphVert::new(i, *v)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_vertices() -> Vec<Vec2> {
        vec![
            Vec2::new(0f32, 0f32),
            Vec2::new(1f32, 1f32),
            Vec2::new(2f32, 2f32),
            Vec2::new(3f32, 1f32),
            Vec2::new(4f32, 0f32),
            Vec2::new(2f32, 1f32),
        ]
    }

    #[test]
    fn vertices_to_heap_test() {
        let mut h = vertices_to_heap(&test_vertices());
        assert_eq!(h.pop().unwrap().0, 2);
        assert_eq!(h.pop().unwrap().0, 1);
        assert_eq!(h.pop().unwrap().0, 5);
        assert_eq!(h.pop().unwrap().0, 3);
        assert_eq!(h.pop().unwrap().0, 0);
        assert_eq!(h.pop().unwrap().0, 4);
    }
}
