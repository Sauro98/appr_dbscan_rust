use crate::utils::*;
use std::collections::HashMap;
use rstar::{RTree, RTreeParams, RStarInsertionStrategy};
use crate::tree_structure::TreeStructure;

/// Parameters euristhically found to male the r-tree construction and query faster
struct LargeNodeParameters<const D:usize>;
impl <const D:usize> RTreeParams for LargeNodeParameters<D> {
    const MAX_SIZE: usize = 4 * D;
    const MIN_SIZE: usize = Self::MAX_SIZE/2;
    const REINSERTION_COUNT: usize = (Self::MAX_SIZE-Self::MIN_SIZE)/2;
    type DefaultInsertionStrategy = RStarInsertionStrategy;

}

#[derive(Clone)]
/// A point in a D dimensional euclidean space that memorizes its
/// status: 'core' or 'non core'
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
/// Informations regarding the cell used in various stages of the approximate DBSCAN
/// algorithm if it is a core cell
pub struct CoreCellInfo <const D: usize>{
    /// The root of the approximate range counting tree built on the core points of the cell
    pub root: TreeStructure<D>,
    /// The index of the cluster where the cell belongs
    pub i_cluster: usize,
    /// The index of the element of union-find structure that references this cell 
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
/// A cell from a grid that partitions the D dimensional euclidean space.
pub struct Cell <const D: usize>{
    /// The index of the intervals of the D dimensional axes where this cell lies
    pub index: CellIndex<D>,
    /// The points from the dataset that lie inside this cell
    pub points: Vec<StatusPoint<D>>,
    /// The list of all the cells that might contain poinst at distance at most 
    /// 'epsilon' from a point in this cell
    pub neighbour_cell_indexes: Vec<CellIndex<D>>,
    /// Keeps track of wether this cell is a core cell or not
    pub is_core: bool,
    /// The additional informations that need to be stored if this cell is indeed a core cell
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

/// A structure that memorizes all non empty cells by their index's hash
pub type CellTable <const D: usize> = HashMap<CellIndex<D>, Cell<D>>;

/// Divides the D dimensional euclidean space in a grid of cells with side length `epsilon\sqrt(D)` and memorizes 
/// the non empty ones in a `CellTable`
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

/// Builds an r-tree on all the non empty cells and executes a query on each one to find the indexes of all their possible neighbour cells
pub fn populate_neighbours<const D: usize>(table: &mut CellTable<D>){
    let rtree: RTree<CellIndexPoint<D>, LargeNodeParameters<D>> = RTree::bulk_load_with_params(table.keys().map(|k| CellIndexPoint{index: *k}).collect());
    /*let mut cell_counter = 0;
    let mut neighbour_counter = 0;
    let mut points_counter = 0;*/
    for (key, cell) in table.iter_mut() {
        //devo filtrare perche' li voglio minori e non minori_uguali
        let neighbours : Vec<CellIndex<D>>= rtree.locate_within_distance(CellIndexPoint{index: key.clone()}, (4 * D) as i64).filter(
            |x| index_distance_sq(&x.index, key) < 4*D
        ).map(|x| x.index).collect();
        /*cell_counter += 1;
        neighbour_counter +=neighbours.len();
        points_counter += cell.points.len();*/
        cell.neighbour_cell_indexes = neighbours;
    }
    //println!("Average number of neighbours: {}",neighbour_counter/cell_counter);
    //println!("Average number of points per cell: {}",points_counter/cell_counter);
    /*for (key, cell) in table.iter_mut() {
        get_neighbours(&cell.index, &mut cell.neighbour_cell_indexes);
    }*/
}

#[cfg(test)]
mod tests;