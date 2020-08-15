use crate::utils::*;
use std::collections::HashMap;

#[derive(Clone)]
pub struct StatusPoint {
    pub point: Vec<f64>,
    pub is_core: bool
}

impl StatusPoint {
    fn new(point: &Vec::<f64>) -> StatusPoint {
        StatusPoint {
            point: point.clone(),
            is_core: false
        }
    }
}

#[derive(Clone)]
pub struct Cell {
    pub index: Vec<i64>,
    pub points: Vec<StatusPoint>,
    pub neighbour_cell_indexes: Vec<Vec<i64>>
}

impl Cell {
    fn new(index_arr: &Vec<i64>) -> Cell{
        let mut cell = Cell {
            index: index_arr.clone(),
            points: Vec::new(),
            neighbour_cell_indexes: Vec::new()
        };
        get_neighbours(index_arr, &mut cell.neighbour_cell_indexes);
        cell
    }
}

pub type CellTable = HashMap<Vec<i64>, Cell>;

pub fn find_cells(points: &Vec<Vec<f64>>, params: &DBSCANParams) -> CellTable {
    let mut table : CellTable = HashMap::with_capacity(params.cardinality as usize);
    for p_i in 0..params.cardinality as usize {
        let curr_point = &points[p_i];
        let index_arr = get_base_cell_index(curr_point, params);
        let cell = table.entry(index_arr.clone())
                    .or_insert(Cell::new(&index_arr));
        cell.points.push(StatusPoint::new(curr_point));
    }
    table
}

#[cfg(test)]
mod tests;