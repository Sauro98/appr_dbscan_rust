use crate::utils::*;
use std::collections::HashMap;
use rstar::RTree;
use crate::tree_structure::TreeStructure;

#[derive(Clone)]
pub struct StatusPoint<const D: usize> {
    pub point: Point<D>,
    pub is_core: bool
}

impl <const D: usize> StatusPoint<D> {
    fn new(point: Point<D>) -> StatusPoint<D> {
        StatusPoint {
            point: point,
            is_core: false
        }
    }
}

#[derive(Clone)]
pub struct CoreCellInfo <const D: usize>{
    pub root: TreeStructure<D>,
    pub i_cluster: usize,
    pub uf_index: usize
}

impl <const D: usize> CoreCellInfo<D>{
    fn new() -> CoreCellInfo<D> {
        CoreCellInfo{
            root: TreeStructure::new_empty(),
            i_cluster: 0,
            uf_index: 0
        }
    }
}

#[derive(Clone)]
pub struct Cell <const D: usize>{
    pub index: CellIndex<D>,
    pub points: Vec<StatusPoint<D>>,
    pub neighbour_cell_indexes: Vec<CellIndex<D>>,
    pub is_core: bool,
    pub core_info: CoreCellInfo<D>,
}

impl <const D: usize> Cell<D> {
    fn new(index_arr: &CellIndex<D>) -> Cell<D>{
        Cell {
            index: index_arr.clone(),
            points: Vec::new(),
            //TODO::size
            neighbour_cell_indexes: Vec::new(),
            is_core: false,
            core_info: CoreCellInfo::new()
        }
        //get_neighbours(index_arr, &mut cell.neighbour_cell_indexes);
    }
}

pub type CellTable <const D: usize> = HashMap<CellIndex<D>, Cell<D>>;

pub fn find_cells<const D: usize>(points: Vec<Point<D>>, params: &DBSCANParams) -> CellTable<D> {
    let mut table : CellTable<D> = CellTable::with_capacity(params.cardinality);
    for p_i in 0..params.cardinality {
        let curr_point = points[p_i];
        let index_arr = get_base_cell_index(&curr_point, params);
        let cell = table.entry(index_arr.clone())
                    .or_insert(Cell::new(&index_arr));
        cell.points.push(StatusPoint::new(curr_point));
    }
    table
}

pub fn populate_neighbours<const D: usize>(table: &mut CellTable<D>){
    let rtree = RTree::bulk_load(table.keys().map(|k| CellIndexPoint{index: *k}).collect());
    let mut cell_counter = 0;
    let mut neighbour_counter = 0;
    for (key, cell) in table.iter_mut() {
        let neighbours : Vec<CellIndex<D>>= rtree.locate_within_distance(CellIndexPoint{index: key.clone()}, (4 * D) as i64).map(|n| n.index).collect();
        cell_counter += 1;
        neighbour_counter +=neighbours.len();
        cell.neighbour_cell_indexes = neighbours;
    }
    println!("Average number of neighbours: {}",neighbour_counter/cell_counter);
}

#[cfg(test)]
mod tests;