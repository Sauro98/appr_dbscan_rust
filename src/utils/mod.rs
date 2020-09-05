use rstar::{Point as RPoint};

/// Type that represents a point with dimensionality D
pub type Point<const D: usize> = [f64;D];
pub type CellIndex<const D: usize> = [i64;D];
pub type CellCenter<const D: usize> = Point<D>;

#[derive(Clone,Copy,PartialEq,Debug)]
/// 
pub struct CellIndexPoint<const D: usize>{
    pub index: CellIndex<D>
}

impl <const D:usize> RPoint for CellIndexPoint<D>{
    
    type Scalar = i64;
    const DIMENSIONS: usize = D;

    fn generate(generator: impl Fn(usize) -> Self::Scalar) -> Self
    {
        let mut r : CellIndexPoint<D> = CellIndexPoint{index: [0;D]};
        for i in 0..D {
            r.index[i] = generator(i);
        }
        r
    }

    fn nth(&self, index: usize) -> Self::Scalar{
        if index < D {
            self.index[index]
        } else {
            unreachable!()
        }
    }

    fn nth_mut(&mut self, index: usize) -> &mut Self::Scalar
    {
        if index < D {
            &mut self.index[index]
        } else {
            unreachable!()
        }
    }
}


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
    for i in 0..D{
        sum += (p[i]-q[i]).powf(2_f64);
    }
    sum.sqrt()
}

pub fn determine_intersection<const D: usize>(q: &Point<D>, params: &DBSCANParams, index_c: &CellIndex<D>, side_size:f64) -> IntersectionType{
    let n_corners = (2_usize.pow(D as u32)) as usize;
    let mut cell_center : CellCenter<D> = [0.0;D];
    for i in 0..D {
        cell_center[i] = index_c[i] as f64 * side_size;
    }
    let corners = get_corners(&cell_center, side_size);
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

fn get_corners<const D: usize>(cell_center: &CellCenter<D>, side_size: f64) -> Vec<Point<D>>{
    let dist = side_size/2.0;
    //Ho 2^d combinazioni. Posso pensare ogni combinazione come un numero binario di d cifre.
    //Immagino di sostituire lo 0 con -dist e l'1 con +dist. Allora posso partire da cell_center
    //e fare la sua somma con ogni numero binario per trovare tutti i vertici
    let top = 2_usize.pow(D as u32);
    let mut corners = Vec::with_capacity(top);
    for bin_rep in 0..top {
        let mut new_corner = cell_center.clone();
        for bit_i in 0..D {
            let mask = 1 << bit_i;
            if bin_rep & mask == 0 {
                new_corner[bit_i] -= dist;
            } else {
                new_corner[bit_i] += dist;
            }
        }
        //println!("{:?}",new_corner);
        corners.push(new_corner);
    }
    corners
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

/*fn get_neighbours_rec<const D: usize>(reference: &CellIndex<D>, index_c: &CellIndex<D>, j: usize, neighbours: &mut Vec<CellIndex<D>>){
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

fn index_distance<const D: usize>(i_1 : &CellIndex<D>, i_2: &CellIndex<D>) -> usize {
    let mut dist : usize = 0;
    for j in 0..i_1.len() {
        dist += (i_1[j] - i_2[j]).pow(2) as usize;
    }
    dist
}
pub fn get_neighbours<const D: usize>(reference: &CellIndex<D>, neighbours: &mut Vec<CellIndex<D>>){
    let new_index = reference.clone();
    get_neighbours_rec(reference, &new_index, 0, neighbours);
}*/

#[cfg(test)]
mod tests;