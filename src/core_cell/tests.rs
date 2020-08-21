use super::*;

#[test]
fn label_points_test() {
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
    let p4 = [l,l];
    let mut points = Vec::with_capacity(4);
    points.push(p1);
    points.push(p2);
    points.push(p3);
    points.push(p4);
    let mut base_table = find_cells(points, &params);
    let mut p_v = label_points(&mut base_table, &params);
    compute_adjacency_lists(&mut base_table, &params, &mut p_v);
    assert_eq!(base_table.len(), 2);   
    let core_cells_count = base_table.values().filter(|x| x.is_core).count();
    assert_eq!(core_cells_count, 2);
    assert_eq!(p_v.all_sets().count(),1);
    for set in p_v.all_sets() {
        assert_eq!(set.count(), 2);
    }
    let p1 = [2.0*l,2.0*l];
    let p2 = [2.0*l,2.0*l];
    let p3 = [2.0*l,2.0*l];
    let p4 = [-5.0*l,-5.0*l];
    let mut points = Vec::with_capacity(4);
    points.push(p1);
    points.push(p2);
    points.push(p3);
    points.push(p4);
    let mut base_table = find_cells(points, &params);
    let p_v = label_points(&mut base_table, &params);
    assert_eq!(base_table.len(), 2);   
    assert_eq!(base_table.values().filter(|x| x.is_core).count(), 1);
    assert_eq!(p_v.all_sets().count(),1);
    for set in p_v.all_sets() {
        assert_eq!(set.count(), 1);
    }


}