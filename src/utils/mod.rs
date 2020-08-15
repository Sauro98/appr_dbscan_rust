pub type Point<const D: usize> = [f64;D];
pub type CellIndex<const D: usize> = [i64;D];
pub type CellCenter<const D: usize> = Point<D>;

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

pub fn euclidean_distance<const D: usize>(p: &Point<D>, q: &Point<D>) -> f64 {
    let mut sum : f64 = 0.0;
    for i in 0..p.len(){
        sum += (p[i]-q[i]).powf(2_f64);
    }
    sum.sqrt()
}

fn get_corners_of_cell<const D: usize>(cell_center: &CellCenter<D>, side_size: f64, j: usize, corners: &mut Vec<Point<D>>){
    let mut new_corner = cell_center.clone();
    if j != D - 1 {
        new_corner[j] += side_size/2.0;
        get_corners_of_cell(&new_corner, side_size, j + 1, corners);
        new_corner[j] -= side_size;
        get_corners_of_cell(&new_corner, side_size, j + 1, corners);
    } else {
        new_corner[j] += side_size/2.0;
        corners.push(new_corner.clone());
        new_corner[j] -= side_size;
        corners.push(new_corner);
    }
}

pub fn get_corners<const D: usize>(cell_center: &CellCenter<D>, side_size: f64, corners: &mut Vec<Point<D>>){
    get_corners_of_cell(cell_center, side_size, 0, corners);
}

pub fn determine_intersection<const D: usize>(q: &Point<D>, params: &DBSCANParams, index_c: &CellIndex<D>, side_size:f64) -> IntersectionType{
    let n_corners = (2_usize.pow(D as u32)) as usize;
    let mut corners : Vec<Point<D>> = Vec::with_capacity(n_corners);
    let mut cell_center : CellCenter<D> = [0.0;D];
    for i in 0..D {
        cell_center[i] = index_c[i] as f64 * side_size;
    }
    get_corners(&cell_center, side_size, &mut corners);
    let appr_dist = (1.0 + params.rho) * params.epsilon;
    let mut appr_in_count : usize = 0;
    let mut out_count : usize = 0;
    for corner in corners {
        let dist = euclidean_distance(q, &corner);
        if dist <= appr_dist {
            appr_in_count += 1;
        } else if dist >= params.epsilon {
            out_count += 1;
        }
        /*if appr_in_count != 0 && out_count != 0 {
            return IntersectionType::Intersecting;
        }*/
    }

    if appr_in_count == n_corners{
        return IntersectionType::FullyCovered
    } else if out_count == n_corners{
        return IntersectionType::Disjoint
    }
    IntersectionType::Intersecting
}

fn index_distance<const D: usize>(i_1 : &CellIndex<D>, i_2: &CellIndex<D>) -> usize {
    let mut dist : usize = 0;
    for j in 0..i_1.len() {
        dist += (i_1[j] - i_2[j]).pow(2) as usize;
    }
    dist
}

fn get_neighbours_rec<const D: usize>(reference: &CellIndex<D>, index_c: &CellIndex<D>, j: usize, neighbours: &mut Vec<CellIndex<D>>){
    let maximum_distance = (D as f64).sqrt().ceil() as i64;
    let mut new_index = index_c.clone(); 
    let j_ind = index_c[j];
    for nval in j_ind - maximum_distance ..= j_ind + maximum_distance {
        new_index[j] = nval;    
        if j < index_c.len() - 1{
            get_neighbours_rec(reference, &new_index, j + 1, neighbours);
        } else {
            if index_distance(reference, &new_index) < 4 * D {
                neighbours.push(new_index.clone());
            }
        }
    }
}

pub fn get_neighbours<const D: usize>(reference: &CellIndex<D>, neighbours: &mut Vec<CellIndex<D>>){
    let new_index = reference.clone();
    get_neighbours_rec(reference, &new_index, 0, neighbours);
}

pub fn get_cell_index<const D: usize>(p: &Point<D>, side_size: f64) -> CellIndex<D>{
    let mut new_index = [0;D];
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

pub fn get_base_cell_index<const D: usize>(p: &Point<D>, params: &DBSCANParams) ->CellIndex<D>{
    get_cell_index(p, params.epsilon/(params.dimensionality as f64).sqrt())
}

#[cfg(test)]
mod tests;