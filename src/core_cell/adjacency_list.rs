use crate::utils::CellIndex;

#[derive(Clone)]
pub struct AdjacencyList <const D: usize>{
    pub vertex: CellIndex<D>,
    pub adjacent_vertices: Vec<CellIndex<D>>
}

impl <const D: usize> AdjacencyList<D> {
    pub fn new(vertex: &CellIndex<D>) -> AdjacencyList<D> {
        AdjacencyList {
            vertex: vertex.clone(),
            adjacent_vertices: Vec::new()
        }
    } 
}