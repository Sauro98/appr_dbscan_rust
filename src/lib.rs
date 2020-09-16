#![feature(min_const_generics)]
pub mod utils;
pub mod tree_structure;
pub mod cell;
pub mod core_cell;
pub mod cluster;
pub mod dbscan;
pub mod data_io;

extern crate partitions;
extern crate rstar;

use utils::{Point};
use data_io::{params_from_file, read_points_from_file};
use dbscan::approximate_dbscan;
use cluster::DBSCANResult;
use std::path::{Path};

/*pub fn test(){
    let mut params = params_from_file("synthetic_db_1.txt");
    assert_eq!(params.dimensionality,5);
    assert_eq!(params.cardinality, 200000);
    params.epsilon = 0.8;
    params.rho = 0.1;
    let points : Vec<Point<5>> = read_points_from_file("synthetic_db_1.txt", &params);
    let res = approximate_dbscan(points, &params);
    assert_eq!(res.len() ,139);
    assert_eq!(res[0].len(), 9722);
}*/

/// Functions that returns the result of the approximate DBSCAN algorithm 
/// executed on the set of points contained in `filename` with the given values of epsilon and rho.
///  
/// # Arguments
/// 
/// * `filename`: the path to the file containing the data points. The file should be formatted with one point per line and the values for each coordinate should be 
///     separated by a white space. Only numerical coordinates values are accepted. 
/// * `epsilon`: the radius for the DBSCAN algorithm. 
/// * `rho`: the approximation factor. The smaller it is the more precise the result. Usual values are 0.1 and 0.01.
/// * 'min_pts': the minimum number of nearby points required by the DBSCAN algorithm to declare an area as 'dense'.
/// 
/// # Constant argument
/// 
/// * `D`: The dimensionality of each point in the data file. If it is not known it can be acquired usinge the [`params_from_file`](data_io/fn.params_from_file.html) function 
///     in the [`data_io`](data_io/index.html) module.
/// 
/// # Return value
/// 
/// This function returns an array of clusters, where each cluster is a vector of the points contained in it. Each point is stored as an array of f64 ([f64;D]).
/// The element at index `0` is the collection of all noise points, while all the other elements are the actual clusters. 
/// 
/// # Example
/// ``` rust
/// extern crate appr_dbscan;
/// use appr_dbscan::do_appr_dbscan_d;
/// use appr_dbscan::cluster::DBSCANResult;
/// 
/// let res : DBSCANResult<2> = do_appr_dbscan_d("./datasets/out_test_1.txt", 0.3, 0.1, 10);
/// let clusters_count = res.len() - 1;
/// let noise_points_count = res[0].len();
/// ```
/// 
pub fn do_appr_dbscan_d<P, const D: usize>(filename: P, epsilon: f64, rho: f64, min_pts: usize) -> DBSCANResult<D> 
where P: AsRef<Path>{
    let mut params = params_from_file(&filename);
    if params.dimensionality != D as u32 {
        panic!("Error: declared point dimensionality is {} but the data file contains points with {} dimensions", D, params.dimensionality);   
    }
    params.epsilon = epsilon;
    params.rho = rho;
    params.min_pts = min_pts;
    let points : Vec<Point<D>> = read_points_from_file(&filename, &params);
    let res = approximate_dbscan(points, &params);
    res
}