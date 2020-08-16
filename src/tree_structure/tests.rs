use super::*;

#[test]
fn build_structure_test() {
    let params = DBSCANParams {
        cardinality: 2,
        dimensionality: 2,
        epsilon: 2.0,
        rho: 0.1,
        min_pts: 0
    };
    let l = params.epsilon / (params.dimensionality as f64).sqrt();
    let q = [l,l];
    let q2 = [-l,l];
    let mut points : Vec<Point<2>> = Vec::with_capacity(2);
    points.push(q);
    points.push(q2);
    let _root = TreeStructure::build_structure(&points, &params);
}

#[test]
fn counting_test(){
    let params = DBSCANParams {
        cardinality: 2,
        dimensionality: 2,
        epsilon: 2.0,
        rho: 0.1,
        min_pts: 0
    };
    let l = params.epsilon / (params.dimensionality as f64).sqrt();
    let q = [l,l];
    let q2 = [-l,l];
    let mut points : Vec<Point<2>> = Vec::with_capacity(2);
    points.push(q.clone());
    points.push(q2.clone());
    let root = TreeStructure::build_structure(&points, &params);
    let central = [0.0,0.0];
    let far = [10.0*l, 10.0*l];
    assert_eq!(root.approximate_range_counting_root(&q, &params),1);
    assert_eq!(root.approximate_range_counting_root(&q2, &params),1);
    assert_eq!(root.approximate_range_counting_root(&central, &params),2);
    assert_eq!(root.approximate_range_counting_root(&far, &params),0);
}