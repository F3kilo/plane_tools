use crate::graph::Graph;
use crate::extended_vec2::ExtVec2;
use std::collections::{BinaryHeap, HashSet};
use std::iter::FromIterator;
use crate::intersect::intersect_point;

fn print_edges(edges: &HashSet<(ExtVec2, ExtVec2)>) {
    println!("\tEdges:");
    for e in edges {
        println!("\t\t{} <---> {}", e.0, e.1);
    }
}

pub fn into_no_intersect(mut graph: Graph<ExtVec2>) -> Graph<ExtVec2> {
    let mut heap = BinaryHeap::from_iter(graph.vertices().cloned());
    let mut edges = HashSet::with_capacity(graph.len());
    while let Some(p) = heap.pop() {
        println!("Start process: {}", p);
        print_edges(&edges);
        let mut s = unchecked_connected(&graph, p, &mut edges);
        println!("\tUnchecked connected: {:?}", s);
        let mb_t_max = highest_ip(p, &s, &edges);
        println!("\tMay be top intersection: {:?}", mb_t_max);
        if let Some(t_max) = mb_t_max {
            split_by_ip(&mut graph, p, &t_max, &mut s, &mut edges, &mut heap);
        }
        for q in s {
            edges.insert((p, q));
        }

        println!("Connects of p: {:?}", graph.connects_of(&p));

        println!("----------------------------------------");
    }
    graph
}

fn unchecked_connected(
    g: &Graph<ExtVec2>,
    p: ExtVec2,
    edges: &mut HashSet<(ExtVec2, ExtVec2)>,
) -> HashSet<ExtVec2> {
    let mut s = g.connects_of(&p).unwrap().clone();
    for q in s.clone() {
        if edges.remove(&(q, p)) {
            s.remove(&q);
        }

        edges.remove(&(p, q));
    }
    s
}

fn highest_ip(
    p: ExtVec2,
    s: &HashSet<ExtVec2>,
    edges: &HashSet<(ExtVec2, ExtVec2)>,
) -> Option<IntersectionPoint> {
    let mut top_ip = None;
    for q in s {
        for (v1, v2) in edges {
            println!("\tIntersection of ({},{}) and ({},{}) is", p, q, v1, v2);
            let mut i = None;
            if q != v2 {
                i = intersect_point((p.0, q.0), (v1.0, v2.0))
            }
            println!("\t\t{:?}", i);
            if let Some(i) = i {
                try_make_higher(&mut top_ip, i.into(), *q, (*v1, *v2));
            }
        }
    }
    top_ip
}

fn try_make_higher(
    i: &mut Option<IntersectionPoint>,
    new_pos: ExtVec2,
    q: ExtVec2,
    edge: (ExtVec2, ExtVec2),
) {
    if let Some(ip) = i {
        if new_pos <= ip.pos {
            return;
        }
    }

    *i = Some(IntersectionPoint { q, edge, pos: new_pos });
}

#[derive(Debug, Clone)]
struct IntersectionPoint {
    q: ExtVec2,
    edge: (ExtVec2, ExtVec2),
    pos: ExtVec2,
}


fn split_by_ip(
    graph: &mut Graph<ExtVec2>,
    p: ExtVec2,
    ip: &IntersectionPoint,
    s: &mut HashSet<ExtVec2>,
    edges: &mut HashSet<(ExtVec2, ExtVec2)>,
    h: &mut BinaryHeap<ExtVec2>,
) {
    println!("\tSplitting by ip: {:?}", ip);
    edges.remove(&ip.edge);
    edges.remove(&(p, ip.q));
    s.remove(&ip.q);
    let is_new = graph.add_vertex(ip.pos);
    println!("\t\tAdd new({}) vertex: {}", is_new, ip.pos);
    graph.remove_edge(&p, &ip.q);
    graph.remove_edge(&ip.edge.0, &ip.edge.1);
    graph.add_edge(&p, &ip.pos);
    graph.add_edge(&ip.pos, &ip.q);
    graph.add_edge(&ip.edge.0, &ip.pos);
    graph.add_edge(&ip.pos, &ip.edge.1);

    edges.insert((p, ip.pos));
    edges.insert((ip.edge.0, ip.pos));

    h.push(ip.pos);
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::Vec2;

    fn square_graph_data() -> (Vec<ExtVec2>, Vec<(ExtVec2, ExtVec2)>, Graph<ExtVec2>) {
        let verts : Vec<ExtVec2> = vec![
            Vec2::new(0f32, 0f32).into(),
            Vec2::new(0f32, 1f32).into(),
            Vec2::new(1f32, 1f32).into(),
            Vec2::new(1f32, 0f32).into(),
            Vec2::new(0.5f32, 0f32).into(),
            Vec2::new(0.5f32, 1f32).into(),
            Vec2::new(1.5f32, 1f32).into(),
            Vec2::new(1.5f32, 0f32).into(),
        ];

        let edges = vec![
            (verts[0], verts[1]),
            (verts[1], verts[2]),
            (verts[2], verts[3]),
            (verts[3], verts[0]),
            (verts[4], verts[5]),
            (verts[5], verts[6]),
            (verts[6], verts[7]),
            (verts[7], verts[4]),
        ];

        let mut expected_result = Graph::with_capacity(verts.len(), 4);
        for vert in &verts {
            expected_result.add_vertex(vert.clone());
        }

        expected_result.add_edge(&verts[0], &verts[1]);
        expected_result.add_edge(&verts[0], &verts[4]);
        expected_result.add_edge(&verts[1], &verts[0]);
        expected_result.add_edge(&verts[1], &verts[5]);
        expected_result.add_edge(&verts[2], &verts[5]);
        expected_result.add_edge(&verts[2], &verts[6]);
        expected_result.add_edge(&verts[2], &verts[3]);
        expected_result.add_edge(&verts[3], &verts[4]);
        expected_result.add_edge(&verts[3], &verts[2]);
        expected_result.add_edge(&verts[3], &verts[7]);
        expected_result.add_edge(&verts[4], &verts[0]);
        expected_result.add_edge(&verts[4], &verts[5]);
        expected_result.add_edge(&verts[4], &verts[3]);
        expected_result.add_edge(&verts[5], &verts[1]);
        expected_result.add_edge(&verts[5], &verts[2]);
        expected_result.add_edge(&verts[5], &verts[4]);
        expected_result.add_edge(&verts[6], &verts[2]);
        expected_result.add_edge(&verts[6], &verts[7]);
        expected_result.add_edge(&verts[7], &verts[3]);
        expected_result.add_edge(&verts[7], &verts[6]);

        (verts, edges, expected_result)
    }



    #[test]
    fn into_no_intersect_test() {
        let (verts, edges, expected_result) = square_graph_data();
        let g = Graph::from_data(verts.clone().into_iter(), edges.into_iter());
        let result = into_no_intersect(g);
        for v in result.vertices() {
            println!("Vertex: {}", v);
            let connected = result.connects_of(v).unwrap();
            for c in connected {
                println!("\tConnected with: {}", c);
            }
        }

        assert_eq!(expected_result, result)
    }
}
