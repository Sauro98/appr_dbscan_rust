use super::*;
use crate::cell::find_cells;
use crate::core_cell::{label_points,compute_adjacency_lists};

#[test]
fn clustering_test() {
    let params = DBSCANParams {
        cardinality: 4,
        dimensionality: 2,
        epsilon: 2.0,
        rho: 0.1,
        min_pts: 2
    };
    let l = params.epsilon / (params.dimensionality as f64).sqrt();
    let p1 = [2.0*l,2.0*l];
    let p2 = [2.0*l,2.0*l];
    let p3 = [2.0*l,2.0*l];
    let p4 = [-5.0*l,-5.0*l];
    let mut points = Vec::with_capacity(4);
    points.push(p1);
    points.push(p2);
    points.push(p3);
    points.push(p4);
    let mut base_table = find_cells(&points, &params);
    let mut p_v = label_points(&mut base_table,&points, &params);
    compute_adjacency_lists(&mut base_table, &points,&params, &mut p_v);
    let mut result = find_connected_components(&mut base_table, &points,p_v);
    assign_border_noise_points(&base_table, &points, &mut result, &params);
    assert_eq!(result.iter().filter(|x| x.is_some()).map(|x| x.unwrap()).max().unwrap().to_owned(), 1);
    assert_eq!(result.iter().filter(|x| x.is_none()).count(), 1);
    assert_eq!(result.iter().filter(|x| x.is_some() && x.unwrap() == 1).count(), 3);
}