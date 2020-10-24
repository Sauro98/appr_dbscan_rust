use crate::cell::{CellTable,StatusPoint};
//use crate::core_cell::core_points_in_range;
use crate::utils::*;
use partitions::PartitionVec;
use ndarray::{Array1};

/// The result of the approximated DBSCAN algorithm always has an element at this index
/// that corresponds to the set of noise points found
pub const NOISE_CLUSTER_INDEX:usize = 0;

/// Explores the union-find structure `part_vec` and puts all core points in the same set in the same clusters and core points from 
/// different sets in different clusters. This function supposes that all possible union operations on `part_vec` were already done.
/// The result of this function is a collection of cluster constructed as described above, and its first element is the (now still empty)
/// set of noise points
pub fn find_connected_components<const D: usize>(cells: &mut CellTable<D>, points: &Vec<Point<D>>, part_vec: PartitionVec<CellIndex<D>>) -> DBSCANResult{
    let mut res = Array1::from_elem(points.len(),None);
    let noise_cluster : Cluster<D> = Vec::new();
    //the noise cluster will be at index 0
    //res.push(noise_cluster);
    let mut current_cluster_i: usize = 1;
    for set in part_vec.all_sets(){
        //let mut new_cluster : Cluster<D> = Vec::new();
        for (_,key) in set {
            let curr_core_cell = cells.get_mut(key).unwrap();
            curr_core_cell.core_info.i_cluster = current_cluster_i;
            for s_point in &curr_core_cell.points {
                if s_point.is_core {
                    //new_cluster.push(points[s_point.point_index].clone());
                    res[s_point.point_index] = Some(current_cluster_i);
                }
            }
        }
        //res.push(new_cluster);
        current_cluster_i += 1;
    }
    res
}

/// Loops through all non core points of the dataset and puts them in the cluster\clusters they belong to. If no such cluster is found then
/// the point is added to the noise points set.
pub fn assign_border_noise_points<const D: usize>(cells: &CellTable<D>, points: &Vec<Point<D>>,clusters: &mut DBSCANResult, params: &DBSCANParams) {
    for cell in cells.values() {
        for s_point in &cell.points {
            if !s_point.is_core {
                assign_border_noise_point(&s_point, &cell.neighbour_cell_indexes, points, clusters, cells, params);
            }
        }
    }
}

/// Evaluates if a single point belongs to one or more clusters or if it is a noise points and adds the point to the cluster/s or set where it belongs.
fn assign_border_noise_point<const D: usize>(s_point: &StatusPoint<D>,neighbours: &Vec<CellIndex<D>>, points: &Vec<Point<D>>, clusters: &mut DBSCANResult, cells: &CellTable<D>, params: &DBSCANParams) {
    //let mut clusters_in : Vec<usize> = Vec::new();
    for n_index in neighbours {
        match cells.get(n_index) {
            Some(curr_cell) => {
                if curr_cell.is_core{
                    //if !clusters_in.contains(&curr_cell.core_info.i_cluster) {
                        if curr_cell.core_info.root.approximate_range_counting_root(
                                &points[s_point.point_index],params) != 0 {    
                            //clusters[curr_cell.core_info.i_cluster].push(point.clone());
                            //clusters_in.push(curr_cell.core_info.i_cluster);
                            clusters[s_point.point_index] = Some(curr_cell.core_info.i_cluster);
                            // assign only to first matching cluster for compatibility
                            break;
                        }
                    //}
                }
            },
            _ => {},
        }
    }
    //if clusters_in.is_empty() {
    //    clusters[NOISE_CLUSTER_INDEX].push(point.clone());
    //}
}

#[cfg(test)]
mod tests;