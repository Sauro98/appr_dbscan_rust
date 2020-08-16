use super::*;
use crate::cell::find_cells;
use crate::core_cell::label_points;

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
    let (mut s_core, mut p_v) = label_points(&mut base_table, &params);
    let mut result = find_connected_components(&mut s_core, &mut p_v);
    assign_border_noise_points(&base_table, &s_core, &mut result, &params);
    assert_eq!(result.len(), 2);
    assert_eq!(result[NOISE_CLUSTER_INDEX].len(), 1);
    assert_eq!(result[1].len(), 3);
}