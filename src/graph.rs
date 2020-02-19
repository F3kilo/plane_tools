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
        connects: impl Iterator<Item = impl Iterator<Item = usize>>,
    ) -> Self {
        let verts: Vec<T> = vertices.collect();
        let mut local_connects: Vec<HashSet<usize>> = Vec::with_capacity(verts.len());
        let mut max_connections = DEFAULT_CONNECTIONS_PER_POINT;
        for vert_connects in connects {
            let vert_connects: HashSet<usize> = vert_connects.collect();
            if max_connections < vert_connects.len() {
                max_connections = verts.len();
            }
            local_connects.push(vert_connects);
        }
        Self {
            verts,
            connects: local_connects,
            edge_per_point_capacity: max_connections,
        }
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
}

impl<T> Index<usize> for Graph<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.verts[index]
    }
}
