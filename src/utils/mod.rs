use rstar::{Point as RPoint};
use ndarray::{Array1,ArrayD};

#[derive(Clone,Copy,PartialEq,Debug)]
/// Mock struct to use RTrees with const generics
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

/// The parameters needed to run the approximate DBSCAN algorithm
pub struct DBSCANParams{
    /// The number of points to cluster
    pub cardinality: usize,
    /// The dimensionality of the space containing the points
    pub dimensionality: u32,
    /// The clustering radius
    pub epsilon: f64,
    /// The approximation factor
    pub rho: f64,
    /// The minimum number of points for density
    pub min_pts: usize
}

#[derive(PartialEq, Debug)]
/// See documentation for the function `utils::determine_intersection`
pub enum IntersectionType{
    FullyCovered,
    Disjoint,
    Intersecting
}

// Computes the euclidean distance between two points in a `D` dimensional space
pub fn euclidean_distance<const D: usize>(p: &Point<D>, q: &Point<D>) -> f64 {
    let mut sum : f64 = 0.0;
    for i in 0..D{
        sum += (p[i]-q[i]).powf(2_f64);
    }
    sum.sqrt()
}

/// Determines the type of intersection between a cell and an approximated ball.
/// The cell is determined by its center and the side of its size.
/// Returns: 
///  * IntersectionType::FullyCovered if the cell is completely contained in a ball with center `q` and radius `epsilon(1 + rho)`;
///  * IntersectionType::Disjoint if the cell is completely outside of a ball with center `q` and radius `epsilon`;
///  * IntersectionType::Intersecting otherwise;
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
        }
        if dist >= params.epsilon {
            out_count += 1;
        }
    }
    if appr_in_count == n_corners{
        return IntersectionType::FullyCovered
    } else if out_count == n_corners{
        return IntersectionType::Disjoint
    }
    IntersectionType::Intersecting
}

/// Gets the coordinates of all the corners (2^D) of a cell given its center points and its side size.
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
        corners.push(new_corner);
    }
    corners
}

/// Gets the indexes of the intervals of the axes in the `D` dimensional space where lies a Cell with side 
/// size equal to `side_size` that contains point `p`
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

/// Gets the indexes of the intervals of the axes in the `D` dimensional space where lies a Cell with side 
/// size equal to `epsilon/sqrt(D)` that contains point `p`
pub fn get_base_cell_index<const D: usize>(p: &Point<D>, params: &DBSCANParams) ->CellIndex<D>{
    get_cell_index(p, params.epsilon/(params.dimensionality as f64).sqrt())
}

/// Gets the euclidean distance to the power of 2 between two arrays representing cell indexes
pub fn index_distance_sq<const D: usize>(i_1 : &CellIndex<D>, i_2: &CellIndex<D>) -> usize {
    let mut dist : usize = 0;
    for j in 0..i_1.len() {
        dist += (i_1[j] - i_2[j]).pow(2) as usize;
    }
    dist
}

/// Array that stores the indexes of the intervals of the axes in the `D` dimensional space 
/// that are occupied by a certain cell
pub type CellIndex<const D: usize> = [i64;D];
/// Type that represent the point in the `D` dimensional space that lays at the center of a cell
pub type CellCenter<const D: usize> = Point<D>;
/// Type that represents a point with dimensionality D
pub type Point<const D: usize> = [f64;D];
/// Collection of points in the same cluster
pub type Cluster <const D: usize> = Vec<Point<D>>;
/// Collection of all the cluster found by the DBSCAN algorithm. Its first element
/// will be the collection of noise points.
pub type DBSCANResult = Array1<Option<usize>>;
//pub type DBSCANResult <const D: usize> = Vec<Cluster<D>>;

/// Point defined as a vector instead of as an array like in `utils::Point`.
/// Used for when dimensionality is not previously known.
/// If dimensionality D is known then using `utils::Point<D>` is preferred 
pub type VectorPoint = Vec<f64>;
/// Cluster redefined to accomodate vector points.
/// If dimensionality D is known then using `utils::Cluster<D>` is preferred 
pub type VectorCluster = Vec<VectorPoint>;
/// Result redefined to accomodate vector clusters
/// If dimensionality D is known then using `utils::DBSCANResult<D>` is preferred 
pub type VectorDBSCANResult = Vec<VectorCluster>;

/// Translates a vector of points represented as vectors in a vector of points represented ad fixed length arrays.
/// Panics if the points do not all have the same length.
pub fn vector_input_to_array_input<const D: usize>(v_in: &Vec<VectorPoint>) -> Vec<Point<D>> {
    if v_in.len() == 0 {
        panic!("Received an unexpected 0 length vector. This should not have happened");
    }
    let mut arr_in = Vec::with_capacity(v_in.len());
    for i in 1..v_in.len() {
        if v_in[i].len() != D {
            panic!("DBSCAN: expected all points to have {} components, but point {} has {} components instead",D, i, v_in[i].len());
        }
        let mut arr_point = [0.0;D];
        for j in 0..D {
            arr_point[j] = v_in[i][j];
        }
        arr_in.push(arr_point);
    }
    arr_in
}

/// Transforms a vector of clusters containing points represented as arrays into a vector of clusters
/// where each point is represented as a vector.
/*pub fn array_res_to_vector_res<const D: usize>(a_res: DBSCANResult<D>) -> VectorDBSCANResult {
    let mut v_res : VectorDBSCANResult = Vec::with_capacity(a_res.len());
    for i in 0..a_res.len() {
        let mut v_cluster = Vec::with_capacity(a_res[i].len());
        for a_point in &a_res[i] {
            v_cluster.push(a_point.to_vec());
        }
        v_res.push(v_cluster);
    }
    v_res
}*/

#[cfg(test)]
mod tests;