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
    let p1 = vec![2.0*l,2.0*l];
    let p2 = vec![2.0*l,2.0*l];
    let p3 = vec![2.0*l,2.0*l];
    let p4 = vec![l,l];
    let mut points : Vec<Vec<f64>> = Vec::with_capacity(4);
    points.push(p1);
    points.push(p2);
    points.push(p3);
    points.push(p4);
    let mut base_table = find_cells(&points, &params);
    let s_core = label_points(&mut base_table, &params);
    assert_eq!(base_table.len(), 2);   
    assert_eq!(s_core.len(), 2);
    for element in s_core.values() {
        assert_eq!(element.adjacency_list.adjacent_vertices.len(), 1);
    }
    let p1 = vec![2.0*l,2.0*l];
    let p2 = vec![2.0*l,2.0*l];
    let p3 = vec![2.0*l,2.0*l];
    let p4 = vec![-5.0*l,-5.0*l];
    let mut points : Vec<Vec<f64>> = Vec::with_capacity(4);
    points.push(p1);
    points.push(p2);
    points.push(p3);
    points.push(p4);
    let mut base_table = find_cells(&points, &params);
    let s_core = label_points(&mut base_table, &params);
    assert_eq!(base_table.len(), 2);   
    assert_eq!(s_core.len(), 1);
    for element in s_core.values() {
        assert_eq!(element.adjacency_list.adjacent_vertices.len(), 0);
    }


}