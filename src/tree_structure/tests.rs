use super::*;

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
    let root1 = TreeStructure::build_structure(vec![q], &params);
    let root2 = TreeStructure::build_structure(vec![q2], &params);
    let central = [0.0,0.0];
    let far = [10.0*l, 10.0*l];
    assert_eq!(root1.approximate_range_counting_root(&q, &params),1);
    assert_eq!(root2.approximate_range_counting_root(&q2, &params),1);
    assert_eq!(root1.approximate_range_counting_root(&central, &params),1);
    assert_eq!(root2.approximate_range_counting_root(&central, &params),1);
    assert_eq!(root1.approximate_range_counting_root(&far, &params),0);
    assert_eq!(root2.approximate_range_counting_root(&far, &params),0);
    assert_eq!(root1.approximate_range_counting_root(&q2, &params),0);
    assert_eq!(root2.approximate_range_counting_root(&q, &params),0);
    assert_eq!(root1.approximate_range_counting_root(&[2.0*l,2.0*l], &params),1);
    assert_eq!(root1.approximate_range_counting_root(&[3.0*l,3.0*l], &params),0);
    assert_eq!(root1.approximate_range_counting_root(&[2.5*l,2.5*l], &params),0);
    assert_eq!(root1.approximate_range_counting_root(&[2.2*l,2.2*l], &params),0);
    assert_eq!(root1.approximate_range_counting_root(&[2.11*l,2.11*l], &params),0);
}