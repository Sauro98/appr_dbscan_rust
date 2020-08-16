use crate::utils::*;
use std::collections::HashMap;

#[derive(Clone)]
pub struct StatusPoint<const D: usize> {
    pub point: Point<D>,
    pub is_core: bool
}

impl <const D: usize> StatusPoint<D> {
    fn new(point: &Point<D>) -> StatusPoint<D> {
        StatusPoint {
            point: point.clone(),
            is_core: false
        }
    }
}

#[derive(Clone)]
pub struct Cell <const D: usize>{
    pub index: CellIndex<D>,
    pub points: Vec<StatusPoint<D>>,
    pub neighbour_cell_indexes: Vec<CellIndex<D>>
}

impl <const D: usize> Cell<D> {
    fn new(index_arr: &CellIndex<D>) -> Cell<D>{
        let mut cell = Cell {
            index: index_arr.clone(),
            points: Vec::new(),
            neighbour_cell_indexes: Vec::new()
        };
        get_neighbours(index_arr, &mut cell.neighbour_cell_indexes);
        cell
    }
}

pub type CellTable <const D: usize> = HashMap<CellIndex<D>, Cell<D>>;

pub fn find_cells<const D: usize>(points: &Vec<Point<D>>, params: &DBSCANParams) -> CellTable<D> {
    let mut table : CellTable<D> = HashMap::with_capacity(params.cardinality as usize);
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