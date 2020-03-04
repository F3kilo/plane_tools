use crate::extended_vec2::ExtVec2;
use crate::graph_split;
use easy_graph::Graph;
use std::collections::HashSet;

pub struct PlanarGraph(Graph<ExtVec2>);

impl PlanarGraph {
    pub fn into_graph(self) -> Graph<ExtVec2> {
        self.0
    }

    /// Returns immutable reference to underlying graph
    pub fn graph(&self) -> &Graph<ExtVec2> {
        &self.0
    }

    /// Calls underlying graph methon [`remove_weak_connected`]
    ///
    /// [`remove_weak_connected`]: ../graph/struct.Graph.html#method.remove_weak_connected
    pub fn remove_weak_connected(&mut self, weak_level: usize) -> HashSet<ExtVec2> {
        self.0.remove_weak_connected(weak_level)
    }

    /// Calls underlying graph methon [`remove_vertex`]
    ///
    /// [`remove_vertex`]: ../graph/struct.Graph.html#method.remove_vertex
    pub fn remove_vertex(&mut self, v: ExtVec2) -> Option<HashSet<ExtVec2>> {
        self.0.remove_vertex(&v)
    }

    /// Calls underlying graph methon [`remove_edge`]
    ///
    /// [`remove_edge`]: ../graph/struct.Graph.html#method.remove_edge
    pub fn remove_edge(&mut self, v1: ExtVec2, v2: ExtVec2) -> bool {
        self.0.remove_edge(&v1, &v2)
    }
}

impl From<Graph<ExtVec2>> for PlanarGraph {
    fn from(g: Graph<ExtVec2>) -> Self {
        Self(graph_split::into_no_intersect(g))
    }
}