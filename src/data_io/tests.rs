use super::*;

#[test]
fn read_file_test(){
    let params = params_from_file("out_test_1.txt");
    assert_eq!(params.dimensionality,2);
    assert_eq!(params.cardinality, 20000);
    let points = read_points_from_file("out_test_1.txt", &params);
    assert_eq!(points.len(), params.cardinality);
    assert_eq!(points[0].len(), params.dimensionality as usize);
    let params = params_from_file("out_test_2.txt");
    assert_eq!(params.dimensionality,3);
    assert_eq!(params.cardinality, 20000);

}