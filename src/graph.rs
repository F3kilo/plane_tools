use std::collections::{HashSet, HashMap};
use std::ops::Index;
use std::hash::Hash;
use std::fmt::Debug;

/// Determine default capacity of connections set for every vertex.
pub const DEFAULT_CONNECTIONS_PER_VERTEX: usize = 4;


/// Data structure that represent generic *vertices* and undirected connections
/// between them - *edges*.
#[derive(Clone, Debug, Default)]
pub struct Graph<T: Hash + Eq> {
    verts: HashMap<T, HashSet<T>>,
    edge_per_vertex_capacity: usize,
}

impl<T: Hash + Eq> Graph<T> {
    /// Creates new empty graph.
    pub fn new() -> Self {
        Self {
            verts: HashMap::new(),
            edge_per_vertex_capacity: DEFAULT_CONNECTIONS_PER_VERTEX,
        }
    }

    /// Creates empty graph with preallocated memory for vertices and edges.
    /// # Arguments:
    /// `vertices_capacity` - capacity for collection of vertices.  
    /// `edge_per_vertex_capacity` - capacity for connections collections for each vertex.
    /// Default value `const DEFAULT_CONNECTIONS_PER_VERTEX: usize = 4`
    pub fn with_capacity(vertices_capacity: usize, edge_per_vertex_capacity: usize) -> Self {
        Self {
            verts: HashMap::with_capacity(vertices_capacity),
            edge_per_vertex_capacity,
        }
    }

    /// Creates graph filled by `vertices` with `edges`.
    /// # Arguments:
    /// `vertices` - iterator of vertices.  
    /// `edges` - iterator of pairs of vertices indices, which must be connected.
    pub fn from_data(
        vertices: impl Iterator<Item=T>,
        edges: impl Iterator<Item=(T, T)>,
    ) -> Self {
        let verts: HashMap<T, HashSet<T>> = vertices
            .map(|v| (v, HashSet::with_capacity(DEFAULT_CONNECTIONS_PER_VERTEX)))
            .collect();
        let len = verts.len();
        let mut temp = Self::with_capacity(len, DEFAULT_CONNECTIONS_PER_VERTEX);
        for v in verts {
            temp.add_vertex(v);
        }

        for (v1, v2) in edges {
            temp.add_edge(v1, v2);
        }

        temp
    }

    /// Adds vertex to graph.
    /// # Arguments:
    /// `vertex` - vertex, that must be added.
    /// # Returns:
    /// Index of added vertex to access to it.
    pub fn add_vertex(&mut self, vertex: T) {
        if self.verts.get(&vertex).is_some() {
            return
        }

        self.verts.push(vertex);
        self.connects
            .push(HashSet::with_capacity(self.edge_per_vertex_capacity));
        new_index
    }
//
//     /// Adds edge to graph.
//     /// # Arguments:
//     /// `v1` - index ofvertex, that must be connected with `v2`.
//     /// `v2` - index ofvertex, that must be connected with `v1`.
//     pub fn add_edge(&mut self, v1: usize, v2: usize) {
//         self.connects[v1].insert(v2);
//         self.connects[v2].insert(v1);
//     }
//
//     /// Removes edge from graph.
//     /// If edge is not present, that function does nothing.
//     /// # Arguments:
//     /// `v1` - index of vertex, that must be disconnected with `v2`.
//     /// `v2` - index of vertex, that must be disconnected with `v1`.
//     pub fn remove_edge(&mut self, v1: usize, v2: usize) {
//         self.connects[v1].remove(&v2);
//         self.connects[v2].remove(&v1);
//     }
//
//     /// Checks if edge, that connects specified vertices, is present.
//     /// Connections are unordered, thats why  always
//     /// `is_connected(v1, v2) == is_connected(v2, v1)`
//     /// # Arguments:
//     /// `v1` - index of first vertex to check.
//     /// `v2` - index of second vertex to check.
//     pub fn is_connected(&self, v1: usize, v2: usize) -> bool {
//         self.connects[v1].contains(&v2)
//     }
//
//     /// Connects of vertices with specified index.
//     /// # Arguments:
//     /// `v` - index of vertex of interest;
//     /// # Returns:
//     /// Set of indices, that connected to `v`.
//     pub fn connects_of(&self, v: usize) -> &HashSet<usize> {
//         &self.connects[v]
//     }
//
//     /// Vector of all current vertices.
//     pub fn vertices(&self) -> &Vec<T> {
//         &self.verts
//     }
//
//     /// Current count of vertices.
//     pub fn len(&self) -> usize {
//         self.verts.len()
//     }
//
//     /// True, if graph does not contain vertices.
//     pub fn is_empty(&self) -> bool {
//         self.verts.is_empty()
//     }
// }
//
// impl<T> Index<usize> for Graph<T> {
//     type Output = T;
//
//     fn index(&self, index: usize) -> &Self::Output {
//         &self.verts[index]
//     }
}
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use glam::Vec2;
//
//     fn test_data() -> (Vec<Vec2>, Vec<(usize, usize)>) {
//         let verts = vec![
//             Vec2::new(0f32, 0f32),
//             Vec2::new(1f32, 1f32),
//             Vec2::new(2f32, 2f32),
//             Vec2::new(3f32, 1f32),
//             Vec2::new(4f32, 0f32),
//         ];
//
//         let conns = vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)];
//         (verts, conns)
//     }
//
//     fn test_graph() -> Graph<Vec2> {
//         let (verts, conns) = test_data();
//         Graph::from_data(verts.into_iter(), conns.clone().into_iter())
//     }
//
//     #[test]
//     fn from_data() {
//         let verts = test_data().0;
//         let g = test_graph();
//
//         assert_eq!(verts.len(), g.len());
//
//         assert_eq!(verts[0], g[0]);
//         assert_eq!(verts[1], g[1]);
//         assert_eq!(verts[2], g[2]);
//         assert_eq!(verts[3], g[3]);
//         assert_eq!(verts[4], g[4]);
//
//         assert!(g.is_connected(0, 1));
//         assert!(g.is_connected(0, 4));
//         assert!(g.is_connected(1, 0));
//         assert!(g.is_connected(1, 2));
//         assert!(g.is_connected(2, 1));
//         assert!(g.is_connected(2, 3));
//         assert!(g.is_connected(3, 2));
//         assert!(g.is_connected(3, 4));
//         assert!(g.is_connected(4, 0));
//     }
//
//     #[test]
//     fn add_vertex() {
//         let mut g = test_graph();
//         let new_vertex = Vec2::new(2f32, 3f32);
//         let new_index = g.add_vertex(new_vertex);
//         assert_eq!(g[new_index], new_vertex);
//         assert!(g.connects_of(new_index).is_empty());
//     }
//
//     #[test]
//     fn add_edge() {
//         let mut g = test_graph();
//         let new_edge = (1, 3);
//         g.add_edge(new_edge.0, new_edge.1);
//         assert!(g.is_connected(new_edge.0, new_edge.1));
//         assert!(g.is_connected(new_edge.1, new_edge.0));
//     }
//
//     #[test]
//     fn remove_edge() {
//         let mut g = test_graph();
//         let removed_edge = (0, 1);
//         g.remove_edge(removed_edge.0, removed_edge.1);
//         assert!(!g.is_connected(removed_edge.0, removed_edge.1));
//         assert!(!g.is_connected(removed_edge.1, removed_edge.0));
//     }
// }
