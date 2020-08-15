pub struct DBSCANParams{
    pub cardinality: usize,
    pub dimensionality: u32,
    pub epsilon: f64,
    pub rho: f64,
    pub min_pts: usize
}

#[derive(PartialEq, Debug)]
pub enum IntersectionType{
    FullyCovered,
    Disjoint,
    Intersecting
}

pub fn euclidean_distance(p: &[f64], q: &[f64]) -> f64 {
    let mut sum : f64 = 0.0;
    for i in 0..p.len(){
        sum += (p[i]-q[i]).powf(2_f64);
    }
    sum.sqrt()
}

fn get_corners_of_cell(cell_center: &[f64], side_size: f64, j: usize, corners: &mut Vec<Vec<f64>>){
    let dim = cell_center.len();
    let mut new_corner = vec![0.0;dim];
    new_corner.copy_from_slice(cell_center);
    if j != dim - 1 {
        new_corner[j] += side_size/2.0;
        get_corners_of_cell(&new_corner, side_size, j + 1, corners);
        new_corner[j] -= side_size;
        get_corners_of_cell(&new_corner, side_size, j + 1, corners);
    } else {
        new_corner[j] += side_size/2.0;
        corners.push(new_corner.clone());
        new_corner[j] -= side_size;
        corners.push(new_corner.clone())
    }
}

pub fn get_corners(cell_center: &[f64], side_size: f64, corners: &mut Vec<Vec<f64>>){
    get_corners_of_cell(cell_center, side_size, 0, corners);
}

pub fn determine_intersection(q: &[f64], params: &DBSCANParams, index_c: &[i64], side_size:f64) -> IntersectionType{
    let dim = q.len();
    let n_corners = (dim as u32 * 2_u32.pow(dim as u32 - 1)) as usize;
    let mut corners : Vec<Vec<f64>> = Vec::with_capacity(n_corners);
    let mut cell_center = vec![0.0; dim];
    for i in 0..dim {
        cell_center[i] = index_c[i] as f64 * side_size;
    }
    get_corners(&cell_center, side_size, &mut corners);
    let appr_dist = (1.0 + params.rho) * params.epsilon;
    let mut appr_in_count : usize = 0;
    let mut out_count : usize = 0;
    for corner in &corners {
        let dist = euclidean_distance(q, &corner);
        if dist <= appr_dist {
            appr_in_count += 1;
        } else if dist >= params.epsilon {
            out_count += 1;
        }
    }

    if appr_in_count == corners.len(){
        return IntersectionType::FullyCovered
    } else if out_count == corners.len(){
        return IntersectionType::Disjoint
    }
    IntersectionType::Intersecting
}

fn index_distance(i_1 : &[i64], i_2: &[i64]) -> u64 {
    let mut dist : u64 = 0;
    for j in 0..i_1.len() {
        dist += (i_1[j] - i_2[j]).pow(2) as u64;
    }
    dist
}

fn get_neighbours_rec(reference: &[i64], index_c: &[i64], j: usize, neighbours: &mut Vec<Vec<i64>>){
    let maximum_distance = (index_c.len() as f64).sqrt().ceil() as i64;
    let dim : u64 = index_c.len() as u64;
    let mut new_index = vec![0; index_c.len()];
    new_index.copy_from_slice(index_c); 
    let j_ind = index_c[j];
    for nval in j_ind - maximum_distance ..= j_ind + maximum_distance {
        new_index[j] = nval;    
        if j < index_c.len() - 1{
            get_neighbours_rec(reference, &new_index, j + 1, neighbours);
        } else {
            if index_distance(reference, &new_index) < 4 * dim {
                neighbours.push(new_index.clone());
            }
        }
    }
}

pub fn get_neighbours(reference: &[i64], neighbours: &mut Vec<Vec<i64>>){
    let mut new_index = vec![0; reference.len()];
    new_index.copy_from_slice(reference);
    get_neighbours_rec(reference, &new_index, 0, neighbours);
}

pub fn get_cell_index(p: &[f64], side_size: f64) -> Vec<i64>{
    let mut new_index = vec![0; p.len()];
    let half_size = side_size/2.0;
    for i in 0..p.len() {
        if p[i] >= (-1.0 * half_size) && p[i] < half_size {
            new_index[i] =  0;
        } else if p[i] > 0.0 {
            new_index[i] = ((p[i] - half_size) / side_size).ceil() as i64;
        } else {
            new_index[i] = -1 + ((p[i] + half_size) / side_size).ceil() as i64;
        }
    }
    new_index
}

pub fn get_base_cell_index(p: &[f64], params: &DBSCANParams) ->Vec<i64>{
    get_cell_index(p, params.epsilon/(params.dimensionality as f64).sqrt())
}

#[cfg(test)]
mod tests;