use std::collections::{BinaryHeap, HashMap,HashSet};
use std::cmp::Ordering;
#[derive(PartialEq,Debug)]
pub struct Graph {
    pub vertices: HashMap<u32, Vec<(u32, f64)>>,
}
impl Graph {
    pub fn new(vertices: usize) -> Self {
        Graph {
            vertices: HashMap::new(),
        }
    }
    pub fn add_edge(&mut self, weight: f64, v: usize, w: usize) {
        self.vertices.entry(v as u32).or_insert(Vec::new()).push((w as u32, weight));
        self.vertices.entry(w as u32).or_insert(Vec::new()).push((v as u32, weight));
    }
    pub fn get_neighbors(&self, vertex: u32) -> Option<&Vec<(u32, f64)>> {
        self.vertices.get(&vertex)
    }
    pub fn degree_centrality(&self) -> HashMap<u32, f64> {
        let mut degree_centrality = HashMap::new();
        let total_nodes = self.vertices.len() as f64;
        for (&node_id, neighbors) in &self.vertices {
            let degree = neighbors.len() as f64;
            degree_centrality.insert(node_id, degree / (total_nodes - 1.0));
        }
        degree_centrality
    }
    pub fn shortest_path_lengths(&self, start: u32) -> HashMap<u32, f64> {
        let mut distances = HashMap::new();
        let mut pq = BinaryHeap::new();
        let mut visited = HashSet::new();
        distances.insert(start, 0.0);
        pq.push(Vertex { id: start, distance: 0.0 });
        while let Some(Vertex { id, distance }) = pq.pop() {
            if visited.contains(&id) {
                continue;
            }
            visited.insert(id);
            if let Some(neighbors) = self.get_neighbors(id) {
                for &(neighbor, weight) in neighbors {
                    let new_distance = distance + weight;
                    if !distances.contains_key(&neighbor) || new_distance < *distances.get(&neighbor).unwrap() {
                        distances.insert(neighbor, new_distance);
                        pq.push(Vertex { id: neighbor, distance: new_distance });
                    }
                }
            }
        }
        distances
    }

    pub fn closeness_centrality(&self) -> HashMap<u32, f64> {
        let mut closeness_centrality = HashMap::new();
        let total_nodes = self.vertices.len() as f64;
        for (&node_id, _) in &self.vertices {
            let shortest_paths = self.shortest_path_lengths(node_id);
            let total_distance: f64 = shortest_paths.values().sum();
            let closeness = if total_distance > 0.0 { (total_nodes - 1.0) / total_distance } else { 0.0 };
            closeness_centrality.insert(node_id, closeness);
        }
        closeness_centrality
    }
}
#[derive(PartialEq, Debug, Clone)]
struct Vertex {
    id: u32,
    distance: f64,
}
impl Eq for Vertex {}
impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.partial_cmp(&self.distance).unwrap()
    }
}
impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
