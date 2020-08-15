use crate::cluster::{DBSCANResult, find_connected_components, assign_border_noise_points};
use crate::cell::{find_cells};
use crate::core_cell::{label_points};
use crate::utils::*;

pub fn approximate_dbscan<const D: usize>(points: &Vec<Point<D>>, params: &DBSCANParams) -> DBSCANResult<D> {
    let mut base_cells = find_cells(points, params);
    println!("Found {:?} cells",base_cells.len());
    let mut s_core = label_points(&mut base_cells, params);
    println!("Of which {:?} are core cells",s_core.len());
    let mut result = find_connected_components(&mut s_core);
    println!("Found {:?} clusters",result.len() - 1);
    assign_border_noise_points(&base_cells, &s_core, &mut result, params);
    println!("Found {:?} noise points",result[0].len());
    result
}

#[cfg(test)]
mod tests;