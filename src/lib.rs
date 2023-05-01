pub mod utils;
mod tree_structure;
mod cell;
mod core_cell;
mod cluster;
pub mod dbscan;
pub mod data_io;

extern crate partitions;
extern crate rstar;

use utils::*;
use data_io::{params_from_file, read_points_from_file};
use dbscan::approximate_dbscan;
use std::path::{Path};

/// Function that returns the result of the approximate DBSCAN algorithm 
/// executed on the set of points contained in `filename` with the given values of epsilon and rho.
///  
/// # Arguments
/// 
/// * `filename`: the path to the file containing the data points. The file should be formatted with one point per line and the values for each coordinate should be 
///     separated by a white space. Only numerical coordinates values are accepted. 
/// * `epsilon`: the radius for the DBSCAN algorithm. 
/// * `rho`: the approximation factor. The smaller it is the more precise the result. Usual values are 0.1 and 0.01.
/// * `min_pts`: the minimum number of nearby points required by the DBSCAN algorithm to declare an area as 'dense'.
/// 
/// # Constant argument
/// 
/// * `D`: The dimensionality of each point in the data file.
/// 
/// # Return value
/// 
/// This function returns a vector of clusters, where each cluster is a vector of the points contained in it. Each point is stored as an array of f64 (`[f64;D]`).
/// The element at index `0` is the collection of all noise points, while all the other elements are the actual clusters. 
/// 
/// # Example
/// ``` rust
/// extern crate appr_dbscan;
/// use appr_dbscan::do_appr_dbscan_file;
/// use appr_dbscan::utils::DBSCANResult;
/// 
/// let res : DBSCANResult<2> = do_appr_dbscan_file("./datasets/out_test_1.txt", 0.3, 0.1, 10);
/// let clusters_count = res.len() - 1;
/// let noise_points_count = res[0].len();
/// ```
/// 
pub fn do_appr_dbscan_file<P, const D: usize>(filename: P, epsilon: f64, rho: f64, min_pts: usize) -> DBSCANResult<D> 
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


/// Function that returns the result of the approximate DBSCAN algorithm 
/// executed on the set of points contained in `points` with the given values of epsilon and rho.
///  
/// # Arguments
/// 
/// * `points`: the vector of points to execute the algorithm on. All points must be arrays of lenght `D` 
/// * `epsilon`: the radius for the DBSCAN algorithm. 
/// * `rho`: the approximation factor. The smaller it is the more precise the result. Usual values are 0.1 and 0.01.
/// * `min_pts`: the minimum number of nearby points required by the DBSCAN algorithm to declare an area as 'dense'.
/// 
/// # Constant argument
/// 
/// * `D`: The dimensionality of each point in the data. 
/// 
/// # Return value
/// 
/// This function returns a vector of clusters, where each cluster is a vector of the points contained in it. Each point is stored as an array of f64 (``[f64;D]``).
/// The element at index `0` is the collection of all noise points, while all the other elements are the actual clusters. 
/// 
/// # Example
/// ``` rust
/// extern crate appr_dbscan;
/// use appr_dbscan::do_appr_dbscan_points;
/// use appr_dbscan::utils::DBSCANResult;
/// 
/// let points = vec![[0.0,0.0],[1.0,1.0],[0.0,1.0],[1.0,0.0],[2.0,1.0],[0.0,2.0],[2.0,1.0],[1.0,1.0]];
/// let res : DBSCANResult<2> = do_appr_dbscan_points(points, 0.3, 0.1, 10);
/// let clusters_count = res.len() - 1;
/// let noise_points_count = res[0].len();
/// ```
/// 
pub fn do_appr_dbscan_points<const D: usize>(points: Vec<Point<D>>, epsilon: f64, rho: f64, min_pts: usize) -> DBSCANResult<D> {
    let params = DBSCANParams{
        dimensionality: D as u32,
        cardinality: points.len(),
        epsilon: epsilon,
        rho: rho,
        min_pts: min_pts
    };
    let res = approximate_dbscan(points, &params);
    res
}

/// Function that returns the result of the approximate DBSCAN algorithm without prior knowledge of the points dimensionality
///, executed on the set of points contained in `filename` with the given values of epsilon and rho.
///  
/// # Arguments
/// 
/// * `filename`: the path to the file containing the data points. The file should be formatted with one point per line and the values for each coordinate should be 
///     separated by a white space. Only numerical coordinates values are accepted. 
/// * `epsilon`: the radius for the DBSCAN algorithm. 
/// * `rho`: the approximation factor. The smaller it is the more precise the result. Usual values are 0.1 and 0.01.
/// * `min_pts`: the minimum number of nearby points required by the DBSCAN algorithm to declare an area as 'dense'.
/// 
/// # Return value
/// 
/// This function returns a vector of clusters, where each cluster is a vector of the points contained in it. Each point is stored as a vector of `f64`, 
/// contrary to the other functions, along with the detected dimensionality of the points inside.
/// The element at index `0` is the collection of all noise points, while all the other elements are the actual clusters. 
/// 
/// # Example
/// ``` rust
/// extern crate appr_dbscan;
/// use appr_dbscan::do_appr_dbscan_auto_dimensionality_file;
/// 
/// let (res,dimensionality) = do_appr_dbscan_auto_dimensionality_file("./datasets/out_test_1.txt", 0.3, 0.1, 10);
/// let clusters_count = res.len() - 1;
/// let noise_points_count = res[0].len();
/// ```
/// 
pub fn do_appr_dbscan_auto_dimensionality_file<P>(filename: P, epsilon: f64, rho: f64, min_pts: usize) -> (VectorDBSCANResult, usize)
where P: AsRef<Path>{
    let params = params_from_file(&filename);
    match params.dimensionality {
        0 => {panic!("There has been an error while reading the data: 0 dimensionality point found");},
        1 => (array_res_to_vector_res::<1>(do_appr_dbscan_file(filename, epsilon, rho, min_pts)),params.dimensionality as usize),
        2 => (array_res_to_vector_res::<2>(do_appr_dbscan_file(filename, epsilon, rho, min_pts)),params.dimensionality as usize),
        3 => (array_res_to_vector_res::<3>(do_appr_dbscan_file(filename, epsilon, rho, min_pts)),params.dimensionality as usize),
        4 => (array_res_to_vector_res::<4>(do_appr_dbscan_file(filename, epsilon, rho, min_pts)),params.dimensionality as usize),
        5 => (array_res_to_vector_res::<5>(do_appr_dbscan_file(filename, epsilon, rho, min_pts)),params.dimensionality as usize),
        6 => (array_res_to_vector_res::<6>(do_appr_dbscan_file(filename, epsilon, rho, min_pts)),params.dimensionality as usize),
        7 => (array_res_to_vector_res::<7>(do_appr_dbscan_file(filename, epsilon, rho, min_pts)),params.dimensionality as usize),
        _ => {panic!("Dimensionalities over 7 are not supported")}
    }
}

/// Function that returns the result of the approximate DBSCAN algorithm without prior knowledge of the points dimensionality
///, executed on the set of points contained in vector `points` with the given values of `epsilon`, `rho` and `min_pts`.
///  
/// # Arguments
/// 
/// * `points`: the vector of points to execute the algorithm on. All points must be vectors of the same length in order to be points from the same space. 
/// * `epsilon`: the radius for the DBSCAN algorithm. 
/// * `rho`: the approximation factor. The smaller it is the more precise the result. Usual values are 0.1 and 0.01.
/// * `min_pts`: the minimum number of nearby points required by the DBSCAN algorithm to declare an area as 'dense'.
/// 
/// # Return value
/// 
/// This function returns a vector of clusters, where each cluster is a vector of the points contained in it. Each point is stored as a vector of `f64`, 
/// contrary to the other functions, along with the detected dimensionality.
/// The element at index `0` is the collection of all noise points, while all the other elements are the actual clusters. 
/// 
/// # Example
/// ``` rust
/// extern crate appr_dbscan;
/// use appr_dbscan::do_appr_dbscan_auto_dimensionality_points;
/// 
/// let points = vec![vec![0.0,0.0],vec![1.0,1.0],vec![0.0,1.0],vec![1.0,0.0],vec![2.0,1.0],vec![0.0,2.0],vec![2.0,1.0],vec![1.0,1.0]];
/// let (res, dimensionality) = do_appr_dbscan_auto_dimensionality_points(points, 0.3, 0.1, 10);
/// let clusters_count = res.len() - 1;
/// let noise_points_count = res[0].len();
/// ```
/// 
pub fn do_appr_dbscan_auto_dimensionality_points(points: Vec<VectorPoint>, epsilon: f64, rho: f64, min_pts: usize) -> (VectorDBSCANResult, usize) {
    if points.len() == 0 {
        return (Vec::new(),0);
    }
    let dimensionality = points[0].len();
    match dimensionality {
        0 => {panic!("There has been an error while reading the data: 0 dimensionality point found");},
        1 => {
            let arr_points = vector_input_to_array_input(points);
            (array_res_to_vector_res::<1>(do_appr_dbscan_points(arr_points, epsilon, rho, min_pts)), dimensionality)
        },
        2 => {
            let arr_points = vector_input_to_array_input(points);
            (array_res_to_vector_res::<2>(do_appr_dbscan_points(arr_points, epsilon, rho, min_pts)), dimensionality)
        },
        3 => {
            let arr_points = vector_input_to_array_input(points);
            (array_res_to_vector_res::<3>(do_appr_dbscan_points(arr_points, epsilon, rho, min_pts)), dimensionality)
        },
        4 => {
            let arr_points = vector_input_to_array_input(points);
            (array_res_to_vector_res::<4>(do_appr_dbscan_points(arr_points, epsilon, rho, min_pts)), dimensionality)
        },
        5 => {
            let arr_points = vector_input_to_array_input(points);
            (array_res_to_vector_res::<5>(do_appr_dbscan_points(arr_points, epsilon, rho, min_pts)), dimensionality)
        },
        6 => {
            let arr_points = vector_input_to_array_input(points);
            (array_res_to_vector_res::<6>(do_appr_dbscan_points(arr_points, epsilon, rho, min_pts)), dimensionality)
        },
        7 => {
            let arr_points = vector_input_to_array_input(points);
            (array_res_to_vector_res::<7>(do_appr_dbscan_points(arr_points, epsilon, rho, min_pts)), dimensionality)
        },
        _ => {panic!("Dimensionalities over 7 are not supported")}
    }
}

