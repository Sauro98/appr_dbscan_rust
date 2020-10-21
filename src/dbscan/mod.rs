use crate::cluster::{find_connected_components, assign_border_noise_points};
use crate::cell::{find_cells, populate_neighbours};
use crate::core_cell::{label_points,compute_adjacency_lists};
use crate::utils::*;
//use std::time::{Instant};

/// Function that runs the approximate DBSCAN algorithm on the given set of points with the given parameters.
/// 
/// # Arguments
/// 
/// * `points` - A vector of `Point` elements to cluster.
/// * `params` - A reference to a `DBSCANParams` struct that holds the clustering parameters
/// 
/// # Return 
/// 
/// An element of type `DBSCANResult`, in which the first cluster contains the noise points. The total number of cluster then
/// is one less than the length of the result returned. 
pub fn approximate_dbscan<const D: usize>(points: Vec<Point<D>>, params: &DBSCANParams) -> DBSCANResult<D> {
    //let tot = Instant::now();
    //let now = Instant::now();
    let mut base_cells = find_cells(points, params);
    //println!("Found {} cells in {} ms",base_cells.len(),now.elapsed().as_millis());
    //let now = Instant::now();
    populate_neighbours(&mut base_cells);
    //println!("Neighbours computed in {} ms",now.elapsed().as_millis());
    //let now = Instant::now();
    let mut part_vec = label_points(&mut base_cells, params);
    //println!("Found {} core cells in {} ms",base_cells.values().filter(|x| x.is_core).count(),now.elapsed().as_millis());
    //let now = Instant::now();
    compute_adjacency_lists(&mut base_cells, params, &mut part_vec);
    //println!("Graph built in {} ms",now.elapsed().as_millis());
    //let now = Instant::now();
    let mut result = find_connected_components(&mut base_cells, part_vec);
    //println!("Found {} clusters in {} ms",result.len() - 1,now.elapsed().as_millis());
    //let now = Instant::now();
    assign_border_noise_points(&base_cells, &mut result, params);
    //println!("Found {} noise points in {} ms",result[0].len(),now.elapsed().as_millis());
    //println!(/*"Completed internal DBSCAN in */"{}"/* milliseconds"*/, tot.elapsed().as_millis());
    /*println!("----------------------CLUSTERS---------------");
    for i in 1..result.len(){
        println!("Cluster #{}: {} points;",i,result[i].len());
    }
    println!("Cluster Noise: {} points;",result[0].len());
    println!("---------------------------------------------");*/
    result
}

#[cfg(test)]
mod tests;