use super::*;
use crate::data_io::*;
use crate::utils::Point;


#[test]
fn dbscan_test_1() {
    let mut params = params_from_file("out_test_1.txt");
    assert_eq!(params.dimensionality,2);
    assert_eq!(params.cardinality, 20000);
    let points : Vec<Point<2>> = read_points_from_file("out_test_1.txt", &params);
    params.epsilon = 0.3;
    params.min_pts = 10;
    params.rho = 0.1;
    let res = approximate_dbscan(points, &params);
    let exp_noise = 0;
    assert_eq!(res[0].len(), exp_noise);
    let exp_clusters = 5;
    assert_eq!(res.len(), exp_clusters + 1);
}

#[test]
fn dbscan_test_2() {
    let mut params = params_from_file("out_test_2.txt");
    assert_eq!(params.dimensionality,3);
    assert_eq!(params.cardinality, 20000);
    let points : Vec<Point<3>> = read_points_from_file("out_test_2.txt", &params);
    params.epsilon = 0.3;
    params.min_pts = 10;
    params.rho = 0.1;
    let res = approximate_dbscan(points, &params);
    let exp_noise = 0;
    assert_eq!(res[0].len(), exp_noise);
    let exp_clusters = 8;
    assert_eq!(res.len(), exp_clusters + 1);
}