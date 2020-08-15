#[derive(Clone)]
pub struct AdjacencyList {
    pub vertex: Vec<i64>,
    pub adjacent_vertices: Vec<Vec<i64>>
}

impl AdjacencyList {
    pub fn new(vertex: &Vec<i64>) -> AdjacencyList {
        AdjacencyList {
            vertex: vertex.clone(),
            adjacent_vertices: Vec::new()
        }
    } 
}