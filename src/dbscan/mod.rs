use crate::cluster::{DBSCANResult, find_connected_components, assign_border_noise_points};
use crate::cell::{find_cells};
use crate::core_cell::{label_points,compute_adjacency_lists};
use crate::utils::*;
use std::time::{Instant};

pub fn approximate_dbscan<const D: usize>(points: Vec<Point<D>>, params: &DBSCANParams) -> DBSCANResult<D> {
    let now = Instant::now();
    let mut base_cells = find_cells(points, params);
    println!("Found {} cells in {} ms",base_cells.len(),now.elapsed().as_millis());
    let now = Instant::now();
    let (mut s_core, mut part_vec) = label_points(&mut base_cells, params);
    println!("Found {} core cells in {} ms",s_core.len(),now.elapsed().as_millis());
    let now = Instant::now();
    compute_adjacency_lists(&mut s_core, params, &mut part_vec);
    println!("Graph built in {} ms",now.elapsed().as_millis());
    let now = Instant::now();
    let mut result = find_connected_components(&mut s_core, part_vec);
    println!("Found {} clusters in {} ms",result.len() - 1,now.elapsed().as_millis());
    let now = Instant::now();
    assign_border_noise_points(&base_cells, &s_core, &mut result, params);
    println!("Found {} noise points in {} ms",result[0].len(),now.elapsed().as_millis());
    result
}

#[cfg(test)]
mod tests;