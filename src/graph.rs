use std::collections::HashMap;
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
}
