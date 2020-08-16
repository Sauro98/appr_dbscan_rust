use super::*;

#[test]
fn find_cells_test(){
    let params = DBSCANParams {
        cardinality: 2,
        dimensionality: 2,
        epsilon: 2.0,
        rho: 0.1,
        min_pts: 0
    };
    let l = params.epsilon / (params.dimensionality as f64).sqrt();
    let q = [l,-l];
    let q2 = [-l,l];
    let mut points = Vec::with_capacity(2);
    points.push(q.clone());
    points.push(q2.clone());
    let base_table = find_cells(&points, &params);
    assert_eq!(base_table.len(), 2);   
}