use crate::cell::{CellTable};
use crate::core_cell::{CoreCellTable};
use crate::utils::*;
use partitions::PartitionVec;

pub type Cluster <const D: usize> = Vec<Point<D>>;
pub type DBSCANResult <const D: usize> = Vec<Cluster<D>>;

pub const NOISE_CLUSTER_INDEX:usize = 0;

pub fn find_connected_components<const D: usize>(s_core: &mut CoreCellTable<D>, part_vec: PartitionVec<CellIndex<D>>) -> DBSCANResult<D>{
    let mut res : DBSCANResult<D> = Vec::new();
    let noise_cluster : Cluster<D> = Vec::new();
    //the noise cluster will be at index 0
    res.push(noise_cluster);
    let mut current_cluster_i: usize = 1;
    for set in part_vec.all_sets(){
        let mut new_cluster : Cluster<D> = Vec::new();
        for (_,key) in set {
            let curr_core_cell = s_core.get_mut(key).unwrap();
            curr_core_cell.i_cluster = current_cluster_i;
            for point in &curr_core_cell.core_points {
                new_cluster.push(point.clone());
            }
        }
        res.push(new_cluster);
        current_cluster_i += 1;
    }
    res
}

pub fn assign_border_noise_points<const D: usize>(cells: &CellTable<D>, s_core: &CoreCellTable<D>,clusters: &mut DBSCANResult<D>, params: &DBSCANParams) {
    for cell in cells.values() {
        for s_point in &cell.points {
            if !s_point.is_core {
                assign_border_noise_point(&s_point.point, &cell.neighbour_cell_indexes, clusters, &s_core, params);
            }
        }
    }
}

fn assign_border_noise_point<const D: usize>(point: &Point<D>,neighbours: &Vec<CellIndex<D>>, clusters: &mut DBSCANResult<D>, s_core: &CoreCellTable<D>, params: &DBSCANParams) {
    let mut clusters_in : Vec<usize> = Vec::new();
    for n_index in neighbours {
        match s_core.get(n_index) {
            Some(core_cell) => {
                if !clusters_in.contains(&core_cell.i_cluster) {
                    if core_cell.root.approximate_range_counting_root(&point,params) != 0 {    
                        clusters[core_cell.i_cluster].push(point.clone());
                        clusters_in.push(core_cell.i_cluster);
                    }
                }
            },
            _ => {},
        }
    }
    if clusters_in.is_empty() {
        clusters[NOISE_CLUSTER_INDEX].push(point.clone());
    }
}

#[cfg(test)]
mod tests;