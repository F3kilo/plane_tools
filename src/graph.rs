use std::collections::HashSet;
use std::ops::Index;

const DEFAULT_CONNECTIONS_PER_POINT: usize = 4;

#[derive(Clone, Debug, Default)]
pub struct Graph<T> {
    verts: Vec<T>,
    connects: Vec<HashSet<usize>>,
    edge_per_point_capacity: usize,
}

impl<T> Graph<T> {
    pub fn new() -> Self {
        Self {
            verts: Vec::new(),
            connects: Vec::new(),
            edge_per_point_capacity: DEFAULT_CONNECTIONS_PER_POINT,
        }
    }

    pub fn with_capacity(points_capacity: usize, edge_per_point_capacity: usize) -> Self {
        Self {
            verts: Vec::with_capacity(points_capacity),
            connects: Vec::with_capacity(points_capacity),
            edge_per_point_capacity,
        }
    }

    pub fn from_data(
        vertices: impl Iterator<Item = T>,
        edges: impl Iterator<Item = (usize, usize)>,
    ) -> Self {
        let verts: Vec<T> = vertices.collect();
        let len = verts.len();
        let mut temp = Self::with_capacity(len, DEFAULT_CONNECTIONS_PER_POINT);
        for v in verts {
            temp.add_vertex(v);
        }

        for (v1, v2) in edges {
            temp.add_edge(v1, v2);
        }

        temp
    }

    pub fn add_vertex(&mut self, vertex: T) -> usize {
        let new_index = self.verts.len();
        self.verts.push(vertex);
        self.connects
            .push(HashSet::with_capacity(self.edge_per_point_capacity));
        new_index
    }

    pub fn add_edge(&mut self, v1: usize, v2: usize) {
        self.connects[v1].insert(v2);
        self.connects[v2].insert(v1);
    }

    pub fn remove_edge(&mut self, v1: usize, v2: usize) {
        self.connects[v1].remove(&v2);
        self.connects[v2].remove(&v1);
    }

    pub fn is_connected(&self, v1: usize, v2: usize) -> bool {
        self.connects[v1].contains(&v2)
    }

    pub fn connects_of(&self, v: usize) -> &HashSet<usize> {
        &self.connects[v]
    }

    pub fn vertices(&self) -> &Vec<T> {
        &self.verts
    }

    pub fn len(&self) -> usize {
        self.verts.len()
    }
}

impl<T> Index<usize> for Graph<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.verts[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::Vec2;

    fn test_data() -> (Vec<Vec2>, Vec<(usize, usize)>) {
        let verts = vec![
            Vec2::new(0f32, 0f32),
            Vec2::new(1f32, 1f32),
            Vec2::new(2f32, 2f32),
            Vec2::new(3f32, 1f32),
            Vec2::new(4f32, 0f32),
        ];

        let conns = vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)];
        (verts, conns)
    }

    #[test]
    fn from_data() {
        let (verts, conns) = test_data();
        let g = Graph::from_data(verts.iter(), conns.clone().into_iter());

        assert_eq!(verts.len(), g.len());

        assert_eq!(verts[0], *g[0]);
        assert_eq!(verts[1], *g[1]);
        assert_eq!(verts[2], *g[2]);
        assert_eq!(verts[3], *g[3]);
        assert_eq!(verts[4], *g[4]);

        assert!(g.is_connected(0, 1));
        assert!(g.is_connected(0, 4));
        assert!(g.is_connected(1, 0));
        assert!(g.is_connected(1, 2));
        assert!(g.is_connected(2, 1));
        assert!(g.is_connected(2, 3));
        assert!(g.is_connected(3, 2));
        assert!(g.is_connected(3, 4));
        assert!(g.is_connected(4, 0));
    }
}
