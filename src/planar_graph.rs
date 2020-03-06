use crate::extended_vec2::ExtVec2;
use easy_graph::{Graph, connected_graph::ConnectedGraph};
use crate::graph_split::into_no_intersect;

pub struct PlanarGraph(ConnectedGraph<ExtVec2>);

impl PlanarGraph {

    /// Make input graph planar (create vertices in intersections) and then split it in connected
    /// planar subgraphs.
    pub fn from_graph(g: Graph<ExtVec2>) -> Vec<Self> {
        let planar = into_no_intersect(g);
        let connected_parts = ConnectedGraph::split_graph(planar);
        connected_parts.into_iter().map(Self).collect()
    }

    /// Convert self into connected graph. Some changes in it can break planar property.
    pub fn into_graph(self) -> ConnectedGraph<ExtVec2> {
        self.0
    }

    /// Returns immutable reference to underlying graph.
    pub fn connected_graph(&self) -> &ConnectedGraph<ExtVec2> {
        &self.0
    }

    /// Returns highest vertex in graph.
    pub fn top_vertex(&self) -> Option<ExtVec2> {
        self.0.graph().vertices().max().cloned()
    }

    /// Calls `ConnectedGraph` method with same name.
    pub fn remove_single_connected(&mut self) {
        self.0.remove_single_connected();
    }
}

#[cfg(test)]
mod tests {
    use easy_graph::Graph;
    use crate::extended_vec2::ExtVec2;
    use glam::Vec2;
    use crate::planar_graph::PlanarGraph;

    fn test_data() -> Graph<ExtVec2> {
        let vs = vertices();

        let edges = vec![
            (vs[0], vs[1]),
            (vs[1], vs[2]),
            (vs[2], vs[3]),
            (vs[3], vs[0]),
            (vs[0], vs[2]),
            (vs[3], vs[1]),
            // Next connected part
            (vs[4], vs[5]),
            (vs[5], vs[6]),
            (vs[6], vs[4]),
            (vs[6], vs[7]),
            (vs[6], vs[8]),
        ];

        Graph::from_data(vs.into_iter(), edges.into_iter())
    }

    fn vertices() -> Vec<ExtVec2> {
        let vs = vec![
            Vec2::new(0f32, 0f32).into(),
            Vec2::new(1f32, 0f32).into(),
            Vec2::new(1f32, 1f32).into(),
            Vec2::new(0f32, 1f32).into(),
            // Next connected part
            Vec2::new(2f32, 2f32).into(),
            Vec2::new(4f32, 2f32).into(),
            Vec2::new(3f32, 3f32).into(),
            Vec2::new(3f32, 1f32).into(),
            Vec2::new(10f32, 10f32).into(),
        ];
        vs
    }



    #[test]
    fn split_into_planar_parts() {
        let (first_part, second_part) = test_connected_parts();
        assert_eq!(first_part.0.graph().len(), 5);
        assert_eq!(second_part.0.graph().len(), 6);
    }

    fn test_connected_parts() -> (PlanarGraph, PlanarGraph) {
        let vs = vertices();
        let mut start_graph = test_data();
        let mut planar_graphs = PlanarGraph::from_graph(start_graph.clone());
        assert_eq!(planar_graphs.len(), 2);
        let mut first_part = planar_graphs.remove(0);
        let mut second_part = planar_graphs.remove(0);
        if second_part.connected_graph().graph().contains(&vs[0]) {
            let temp = second_part;
            second_part = first_part;
            first_part = temp;
        }
        (first_part, second_part)
    }

    #[test]
    fn remove_single_connected() {
        let vs = vertices();
        let (mut first_part,mut second_part) = test_connected_parts();
        assert_eq!(first_part.0.graph().len(), 5);
        first_part.remove_single_connected();
        assert_eq!(first_part.0.graph().len(), 5);

        assert_eq!(second_part.0.graph().len(), 6);
        assert!(second_part.0.graph().contains(&vs[7]));
        assert!(second_part.0.graph().contains(&vs[8]));
        second_part.remove_single_connected();
        assert_eq!(second_part.0.graph().len(), 4);
        assert!(!second_part.0.graph().contains(&vs[7]));
        assert!(!second_part.0.graph().contains(&vs[8]));
    }

    #[test]
    fn top_vertex() {
        let vs = vertices();
        let (first_part, mut second_part) = test_connected_parts();
        assert_eq!(first_part.top_vertex().unwrap(), vs[3]);
        assert_eq!(second_part.top_vertex().unwrap(), vs[8]);
        second_part.remove_single_connected();
        assert_eq!(second_part.top_vertex().unwrap(), vs[6]);
    }
}