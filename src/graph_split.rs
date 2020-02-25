use crate::graph::Graph;
use crate::height_ord_vec2::HeightOrdVec2;
use crate::intersect::intersect_point;
use crate::vec2cmp;
use glam::Vec2;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::iter::FromIterator;

pub struct NoIntersectGraph {
    graph: Graph<Vec2>,
    heap: BinaryHeap<HeightOrdGraphVert>,
    edges: HashSet<(usize, usize)>,
}

impl NoIntersectGraph {
    pub fn new(original_graph: Graph<Vec2>) -> Self {
        Self {
            heap: BinaryHeap::from_iter(
                verts
                    .iter()
                    .enumerate()
                    .map(|(i, v)| HeightOrdGraphVert::new(i, *v)),
            ),
            edges: HashSet::with_capacity(original_graph.len() * 2),
            graph: original_graph,
        }
    }

    pub fn split(mut self) -> Graph<Vec2> {
        while let Some(current_vertex) = self.heap.pop() {
            let current_vertex = current_vertex.0;
            let lc = self.lower_connections(current_vertex);
            let mb_highest_ip = self.highest_intersection_point(current_vertex, &lower_connections);
            if let Some(highest_ip) = mb_highest_ip {
                split_by_intersection(&mut g, p, &t_max, &mut s, &mut edges, &mut heap);
            }
        }

        self.graph
    }

    fn lower_connections(mut self, current_vertex: usize) -> HashSet<usize> {
        self.graph.connects_of(current_vertex).iter()
            .filter(|lc| !self.edges.remove(*(lc, current_vertex))).collect()
    }

    fn highest_intersection_point(self, current_vertex: usize, lower_connections: &HashSet<usize>) -> Option<IntersectionPoint> {
        let mut top_intersection_point = None;
        for lc in lower_connections {
            for (v1, v2) in self.edges {
                let mut i = None;
                let current_line_segment = (self.graph[current_vertex], self.graph[*lc]);
                let edge_line_segment = (self.graph[v1], self.graph[v2]);
                if *lc != v2 {
                    intersect_position = intersect_point(current_line_segment, edge_line_segment);
                }
                try_make_higher(&mut top_intersection_point,
                                intersect_position,
                                *lc,
                                (*v1, *v2),
                );
            }
        }
        top_intersection_point
    }

    fn split_by_intersection(
        current_point_index: usize,
        intersection_point: &IntersectionPoint,
        
    ) {

    }
}


pub fn into_no_intersect(mut g: Graph<Vec2>) -> Graph<Vec2> {
    let mut heap = vertices_to_heap(g.vertices());
    let mut edges: HashSet<(usize, usize)> = HashSet::new();
    while let Some(p) = heap.pop() {
        let p = p.0;
        println!("P={}", p);
        println!("Total Edges: {:?}", edges);
        let mut s = unchecked_edges(&g, p, &mut edges);
        let mb_t_max = highest_intersection_point(&g, p, &s, &edges);
        if let Some(t_max) = mb_t_max {
            split_by_intersection(&mut g, p, &t_max, &mut s, &mut edges, &mut heap);
        }
        edges.extend(s.into_iter().map(|q| (p, q)));
    }
    g
}

fn split_by_intersection(
    g: &mut Graph<Vec2>,
    p: usize,
    t: &IntersectionPoint,
    s: &mut HashSet<usize>,
    edges: &mut HashSet<(usize, usize)>,
    h: &mut BinaryHeap<HeightOrdGraphVert>,
) {
    edges.remove(&t.edge);
    edges.remove(&(p, t.q));
    s.remove(&t.q);
    let i = g.add_vertex(t.position);
    g.remove_edge(p, t.q);
    g.remove_edge(t.edge.0, t.edge.1);
    g.add_edge(p, i);
    g.add_edge(i, t.q);
    g.add_edge(t.edge.0, i);
    g.add_edge(i, t.edge.1);
    edges.insert((p, i));
    edges.insert((t.edge.0, i));
    h.push(HeightOrdGraphVert(i, t.position.into()));
}

#[derive(Debug)]
struct IntersectionPoint {
    q: usize,
    edge: (usize, usize),
    position: Vec2,
}

fn highest_intersection_point(
    g: &Graph<Vec2>,
    p: usize,
    s: &HashSet<usize>,
    edges: &HashSet<(usize, usize)>,
) -> Option<IntersectionPoint> {
    let mut top_intersection_point = None;
    println!("Edges: {:?}", edges);
    for q in s {
        for (v1, v2) in edges {
            let mut i = None;
            if q != v2 {
                i = intersect_point((g[p], g[*q]), (g[*v1], g[*v2]))
            }
            println!(
                "IntersectionPoint: {:?} between edges: {:?}, {:?}",
                i,
                (p, *q),
                (*v1, *v2)
            );
            try_make_higher(&mut top_intersection_point, i, *q, (*v1, *v2));
        }
    }
    println!("Top IntersectionPoint: {:?}", top_intersection_point);
    top_intersection_point
}

fn try_make_higher(
    current_intersection_point: &mut Option<IntersectionPoint>,
    new_intersection_point_pos: Option<Vec2>,
    second_point_index: usize,
    edge: (usize, usize),
) {
    if let Some(pos) = new_intersection_point_pos {
        if let Some(ip) = current_intersection_point {
            if vec2cmp::cmp_y(ip.position, pos) == Ordering::Greater {
                *current_intersection_point = Some(IntersectionPoint { q: second_point_index, edge, position: pos });
            }
        } else {
            *current_intersection_point = Some(IntersectionPoint { q: second_point_index, edge, position: pos });
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

    fn square_graph_data() -> (Vec<Vec2>, Vec<(usize, usize)>) {
        let verts = vec![
            Vec2::new(0f32, 0f32),
            Vec2::new(0f32, 1f32),
            Vec2::new(1f32, 1f32),
            Vec2::new(1f32, 0f32),
            Vec2::new(0.5f32, 0f32),
            Vec2::new(0.5f32, 1f32),
            Vec2::new(1.5f32, 1f32),
            Vec2::new(1.5f32, 0f32),
        ];

        let edges = vec![
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 0),
            (4, 5),
            (5, 6),
            (6, 7),
            (7, 4),
        ];
        (verts, edges)
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

    // #[test]
    // fn into_no_intersect_test() {
    //     let g_data = square_graph_data();
    //     let g = Graph::from_data(g_data.0.clone().into_iter(), g_data.1.clone().into_iter());
    //     let result = into_no_intersect(g);
    //     println!("Result Graph: {:#?}", result);
    // }
}
