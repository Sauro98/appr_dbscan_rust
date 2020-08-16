use super::*;

#[test]
fn euclidean_distance_test() {
    let p = [0.0,0.0];
    let q = [2.0,2.0];
    assert_eq!(euclidean_distance(&p,&q), 2_f64 * 2_f64.sqrt());
}

#[test]
fn get_corners_test(){
    let side_size = 2.0;
    let cell_center = [0.0,0.0];
    let corners = get_corners(&cell_center, side_size);
    //--0
    assert_eq!(corners[0][0], -1.0);
    assert_eq!(corners[0][1], -1.0);
    //--1
    assert_eq!(corners[1][0], 1.0);
    assert_eq!(corners[1][1], -1.0);
    //--2
    assert_eq!(corners[2][0], -1.0);
    assert_eq!(corners[2][1], 1.0);
    //--3
    assert_eq!(corners[3][0], 1.0);
    assert_eq!(corners[3][0], 1.0);

    let cell_center = [0.0, 0.0, 0.0];
    let corners = get_corners(&cell_center, side_size);

    //--0
    assert_eq!(corners[0][0], -1.0);
    assert_eq!(corners[0][1], -1.0);
    assert_eq!(corners[0][2], -1.0);
    //--1
    assert_eq!(corners[1][0], 1.0);
    assert_eq!(corners[1][1], -1.0);
    assert_eq!(corners[1][2], -1.0);
    //--2
    assert_eq!(corners[2][0], -1.0);
    assert_eq!(corners[2][1], 1.0);
    assert_eq!(corners[2][2], -1.0);
    //--3
    assert_eq!(corners[3][0], 1.0);
    assert_eq!(corners[3][1], 1.0);
    assert_eq!(corners[3][2], -1.0);
    //--4
    assert_eq!(corners[4][0], -1.0);
    assert_eq!(corners[4][1], -1.0);
    assert_eq!(corners[4][2], 1.0);
    //--5
    assert_eq!(corners[5][0], 1.0);
    assert_eq!(corners[5][1], -1.0);
    assert_eq!(corners[5][2], 1.0);
    //--6
    assert_eq!(corners[6][0], -1.0);
    assert_eq!(corners[6][1], 1.0);
    assert_eq!(corners[6][2], 1.0);
    //--7
    assert_eq!(corners[7][0], 1.0);
    assert_eq!(corners[7][1], 1.0);
    assert_eq!(corners[7][2], 1.0);
}

#[test]
fn determine_intersection_test() {
    let params = DBSCANParams {
        cardinality: 1,
        dimensionality: 2,
        epsilon: 2.0,
        rho: 0.1,
        min_pts: 0
    };
    let l = params.epsilon/(params.dimensionality as f64).sqrt();
    let q = [l/2.0, (3.0/2.0) * l];
    let cell_index_1 = [0,1];
    let cell_index_2 = [1,1];
    let cell_index_3 = [0,2];
    let cell_index_4 = [1,2];
    let expected_type = IntersectionType::FullyCovered;
    let intersection = determine_intersection(&q, &params, &cell_index_1, l);
    assert_eq!(intersection, expected_type);
    let intersection = determine_intersection(&q, &params, &cell_index_2, l);
    assert_eq!(intersection, expected_type);
    let intersection = determine_intersection(&q, &params, &cell_index_3, l);
    assert_eq!(intersection, expected_type);
    let intersection = determine_intersection(&q, &params, &cell_index_4, l);
    assert_eq!(intersection, expected_type);
    let cell_index_1 = [-1,1];
    let cell_index_2 = [-1,2];
    let cell_index_3 = [2,2];
    let cell_index_4 = [2,1];
    let expected_type = IntersectionType::Intersecting;
    let intersection = determine_intersection(&q, &params, &cell_index_1, l);
    assert_eq!(intersection, expected_type);
    let intersection = determine_intersection(&q, &params, &cell_index_2, l);
    assert_eq!(intersection, expected_type);
    let intersection = determine_intersection(&q, &params, &cell_index_3, l);
    assert_eq!(intersection, expected_type);
    let intersection = determine_intersection(&q, &params, &cell_index_4, l);
    assert_eq!(intersection, expected_type);
    let cell_index_1 = [3,3];
    let cell_index_2 = [3,2];
    let cell_index_3 = [-2,2];
    let cell_index_4 = [-2,1];
    let expected_type = IntersectionType::Disjoint;
    let intersection = determine_intersection(&q, &params, &cell_index_1, l);
    assert_eq!(intersection, expected_type);
    let intersection = determine_intersection(&q, &params, &cell_index_2, l);
    assert_eq!(intersection, expected_type);
    let intersection = determine_intersection(&q, &params, &cell_index_3, l);
    assert_eq!(intersection, expected_type);
    let intersection = determine_intersection(&q, &params, &cell_index_4, l);
    assert_eq!(intersection, expected_type);
}

#[test]
fn get_neighbours_test(){
    let cell_index = [0,0];
    let mut neighbours = Vec::with_capacity(21);
    get_neighbours(&cell_index, &mut neighbours);
    assert_eq!(neighbours.len(), 21);
}

#[test]
fn get_cell_index_test() {
    let l = 15.0;
    let point = [0.0, l];
    let index = get_cell_index(&point, l);
    assert_eq!(index[0], 0);
    assert_eq!(index[1], 1);
    let point = [1.5 * l, -1.0 * l];
    let index = get_cell_index(&point, l);
    assert_eq!(index[0], 1);
    assert_eq!(index[1], -1);
    let point = [-1.5 * l, 2.5 * l];
    let index = get_cell_index(&point, l);
    assert_eq!(index[0], -2);
    assert_eq!(index[1], 2);
    let point = [2.0 * l, 2.0 * l];
    let index = get_cell_index(&point, l);
    assert_eq!(index[0], 2);
    assert_eq!(index[1], 2);
    let point = [-0.5 * l, -0.5 * l];
    let index = get_cell_index(&point, l);
    assert_eq!(index[0], 0);
    assert_eq!(index[1], 0);
    
}