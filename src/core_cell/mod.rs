use crate::cell::*;
use crate::utils::*;
use std::collections::HashMap;
use crate::tree_structure::TreeStructure;
use partitions::PartitionVec;

#[derive(Clone)]
pub struct CoreCell <const D: usize>{
    pub core_points: Vec<Point<D>>,
    pub neighbour_cell_indexes: Vec<CellIndex<D>>,
    pub root: TreeStructure<D>,
    pub i_cluster: usize,
    uf_index: usize
}

impl <const D: usize> CoreCell<D> {
    pub fn new(neighbour_cell_indexes: Vec<CellIndex<D>>, uf_index: usize) -> CoreCell<D> {
        CoreCell {
            core_points: Vec::new(),
            neighbour_cell_indexes: neighbour_cell_indexes,
            root: TreeStructure::new(0,&[0;D],0,0.0),
            i_cluster: 0,
            uf_index: uf_index
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

pub fn label_points<const D: usize>(cells: &mut CellTable<D>, params: &DBSCANParams) -> (CoreCellTable<D>, PartitionVec<CellIndex<D>>) {
    let mut part_vec : PartitionVec<CellIndex<D>> = PartitionVec::with_capacity(params.cardinality);
    let mut s_core : CoreCellTable<D> = HashMap::with_capacity(params.cardinality as usize);
    let cells_cloned = cells.clone();
    for cell in cells.values_mut() {
        if cell.points.len() >= params.min_pts {
            label_dense_cell(cell, &mut s_core, params, &mut part_vec);
        } else {
            label_sparse_cell(&cells_cloned, cell, &mut s_core, params, &mut part_vec)
        }
    }
    (s_core, part_vec)
}

fn label_dense_cell<const D: usize>(cell: &mut Cell<D>, s_core: &mut CoreCellTable<D>, params: &DBSCANParams, uf_str: &mut PartitionVec<CellIndex<D>>){
    let mut core_cell = CoreCell::new(cell.neighbour_cell_indexes.clone(),uf_str.len());
    uf_str.push(cell.index);
    for mut s_point in &mut cell.points {
        s_point.is_core = true;
        core_cell.core_points.push(s_point.point.clone())
    }
    core_cell.root = TreeStructure::build_structure(&core_cell.core_points, params);
    s_core.insert(cell.index.clone(), core_cell);
}


fn label_sparse_cell<const D: usize>(cells_c: &CellTable<D>,curr_cell: &mut Cell<D>, s_core: &mut CoreCellTable<D>, params: &DBSCANParams, uf_str: &mut PartitionVec<CellIndex<D>>){
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
            let tmp_ind = curr_cell.index.clone();
            let tmp_n_i = curr_cell.neighbour_cell_indexes.clone();
            let core_cell = s_core.entry(curr_cell.index.clone())
                .or_insert_with(||{
                    uf_str.push(tmp_ind);
                    CoreCell::new(tmp_n_i, uf_str.len() - 1)
                });
            core_cell.core_points.push(s_point.point.clone());
        }
    }
    if is_core_cell {
        let mut core_cell = s_core.get_mut(&curr_cell.index).unwrap();
        core_cell.root = TreeStructure::build_structure(&core_cell.core_points, params);
    }
}

pub fn compute_adjacency_lists<const D: usize>(s_core:  &mut CoreCellTable<D>, params: &DBSCANParams, part_vec: &mut PartitionVec<CellIndex<D>>){
    for core_cell in s_core.values() {
        for n_index in &core_cell.neighbour_cell_indexes {
            match s_core.get(n_index) {
                Some(neighbour) => {
                    if part_vec.same_set(core_cell.uf_index, neighbour.uf_index){
                        continue;
                    }
                    for point in &core_cell.core_points {
                        if neighbour.root.approximate_range_counting_root(point, params) != 0 {
                            part_vec.union(core_cell.uf_index, neighbour.uf_index);
                            break;
                        }
                    }
                },
                _ => {}
            }
        }
    }
}


#[cfg(test)]
mod tests;