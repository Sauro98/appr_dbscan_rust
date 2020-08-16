use crate::cell::*;
use crate::utils::*;
use std::collections::HashMap;
mod adjacency_list;
use adjacency_list::AdjacencyList;
use crate::tree_structure::TreeStructure;

#[derive(Clone)]
pub struct CoreCell <const D: usize>{
    pub core_points: Vec<Point<D>>,
    pub neighbour_cell_indexes: Vec<CellIndex<D>>,
    pub root: TreeStructure<D>,
    pub visited: bool,
    pub i_cluster: usize,
    pub adjacency_list: AdjacencyList<D>
}

impl <const D: usize> CoreCell<D> {
    pub fn new(neighbour_cell_indexes: &Vec<CellIndex<D>>) -> CoreCell<D> {
        CoreCell {
            core_points: Vec::new(),
            neighbour_cell_indexes: neighbour_cell_indexes.clone(),
            root: TreeStructure::new(0,&[0;D],0,0.0),
            visited: false,
            i_cluster: 0,
            adjacency_list: AdjacencyList::new(&[0;D])
        }
    }
}

pub type CoreCellTable <const D: usize> = HashMap<CellIndex<D>, CoreCell<D>>;

fn points_in_range<const D: usize>(point: &Point<D>, cell: &Cell<D>, epsilon: f64) -> usize{
    let mut cnt : usize = 0;
    for s_point in &cell.points {
        if euclidean_distance(point, &s_point.point) <= epsilon {
            cnt += 1;
        }
    }
    cnt
}

fn is_same_index<const D: usize>(i1: &CellIndex<D>, i2: &CellIndex<D>) -> bool {
    for i in 0..D {
        if i1[i] != i2[i] {
            return false;
        }
    }
    true
}

pub fn label_points<const D: usize>(cells: &mut CellTable<D>, params: &DBSCANParams) -> CoreCellTable<D> {
    let mut s_core : CoreCellTable<D> = HashMap::with_capacity(params.cardinality as usize);
    let cells_cloned = cells.clone();
    for cell in cells.values_mut() {
        if cell.points.len() >= params.min_pts {
            label_dense_cell(cell, &mut s_core, params);
        } else {
            label_sparse_cell(&cells_cloned, cell, &mut s_core, params)
        }
    }
    s_core
}

fn label_dense_cell<const D: usize>(cell: &mut Cell<D>, s_core: &mut CoreCellTable<D>, params: &DBSCANParams){
    let mut core_cell = CoreCell::new(&cell.neighbour_cell_indexes);
    for mut s_point in &mut cell.points {
        s_point.is_core = true;
        core_cell.core_points.push(s_point.point.clone())
    }
    core_cell.root = TreeStructure::build_structure(&core_cell.core_points, params);
    s_core.insert(cell.index.clone(), core_cell);
}


fn label_sparse_cell<const D: usize>(cells_c: &CellTable<D>,curr_cell: &mut Cell<D>, s_core: &mut CoreCellTable<D>, params: &DBSCANParams){
    let mut is_core_cell = false;
    let len = curr_cell.points.len();
    for mut s_point in &mut curr_cell.points {
        let mut tot_pts = len;
        for n_index in &curr_cell.neighbour_cell_indexes {
            if !is_same_index(&curr_cell.index, n_index) {
                match cells_c.get(n_index) {
                    Some(n_cell) => {
                        tot_pts += points_in_range(&s_point.point, n_cell, params.epsilon) 
                    },
                    _ => {}
                }
            }
        }
        if tot_pts >= params.min_pts {
            is_core_cell = true;
            s_point.is_core = true;
            let core_cell = s_core.entry(curr_cell.index.clone())
                .or_insert(CoreCell::new(&curr_cell.neighbour_cell_indexes));
            core_cell.core_points.push(s_point.point.clone());
        }
    }
    if is_core_cell {
        let mut core_cell = s_core.get_mut(&curr_cell.index).unwrap();
        core_cell.root = TreeStructure::build_structure(&core_cell.core_points, params);
    }
}

pub fn compute_adjacency_lists<const D: usize>(s_core:  &mut CoreCellTable<D>, params: &DBSCANParams) {
    let s_core_cloned = s_core.clone();
    for (key, core_cell) in s_core.iter_mut() {
        core_cell.adjacency_list = find_edges_of_cell(key, &s_core_cloned, params);
    }
}

fn find_edges_of_cell<const D: usize>(index_c : &CellIndex<D>, s_core: &CoreCellTable<D>, params: &DBSCANParams) -> AdjacencyList<D> {
    let mut list = AdjacencyList::new(index_c);
    let curr_cell = s_core.get(index_c).unwrap();
    for n_index in &curr_cell.neighbour_cell_indexes {
        if is_same_index(n_index, index_c) {
            continue;
        }
        match s_core.get(n_index) {
            Some(neighbour) => {
                for point in &curr_cell.core_points {
                    if neighbour.root.approximate_range_counting_root(point, params) != 0 {
                        list.adjacent_vertices.push(n_index.clone());
                        break;
                    }
                }
            },
            _ => {}
        }
    }
    list
}

#[cfg(test)]
mod tests;