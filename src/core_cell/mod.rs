use crate::cell::*;
use crate::utils::*;
use crate::tree_structure::TreeStructure;
use partitions::PartitionVec;


/// Counts the points in `cell` that are at distance at most `epsilon` from `point`.
/// The distance used is the euclidean one.
pub fn points_in_range<const D: usize>(point: &Point<D>, cell: &Cell<D>, epsilon: f64) -> usize{
    let mut cnt : usize = 0;
    for s_point in &cell.points {
        if euclidean_distance(point, &s_point.point) <= epsilon {
            cnt += 1;
        }
    }
    cnt
}

/*pub fn core_points_in_range<const D: usize>(point: &Point<D>, cell: &Cell<D>, epsilon: f64) -> usize{
    let mut cnt : usize = 0;
    for s_point in cell.points.iter().filter(|x| x.is_core) {
        if euclidean_distance(point, &s_point.point) <= epsilon {
            cnt += 1;
        }
    }
    cnt
}*/

/// Equality function for type CellIndex<D>
fn is_same_index<const D: usize>(i1: &CellIndex<D>, i2: &CellIndex<D>) -> bool {
    for i in 0..D {
        if i1[i] != i2[i] {
            return false;
        }
    }
    true
}

/// Function that decides which points from each cell are core points and which cells are core cells.
/// 
/// # Arguments:
/// 
/// * `cells`: The non empty cells obtained from partitioning the `D` dimensional euclidean space
/// * `params`: the DBSCAN algorithm parameters
/// 
/// # Return
/// 
/// A union-find structure that contains all and only the core cells found from `cells`. Each cell in `cells` that has been 
/// labeled as a core cell will keep a reference to its index inside the union-find structure
pub fn label_points<const D: usize>(cells: &mut CellTable<D>, params: &DBSCANParams) -> PartitionVec<CellIndex<D>> {
    //The union find structure will contain the core cell that are found, that are for sure at most in the same number 
    //as the non core cells
    let mut part_vec : PartitionVec<CellIndex<D>> = PartitionVec::with_capacity(cells.len());
    let cells_cloned = cells.clone();
    for cell in cells.values_mut() {
        if cell.points.len() >= params.min_pts {
            label_dense_cell(cell, params, &mut part_vec);
        } else {
            label_sparse_cell(&cells_cloned, cell, params, &mut part_vec)
        }
    }
    part_vec
}

/// Function to label as a core cell the cells that have at least 'MinPts' points inside. Sets also the status of
/// all the points in `cell` to 'core'. The cell is then added to the union-find structure `uf_str` and its index inside
/// the structure is memorized in the cell. An approximate range counting structure is then built on the core points and 
/// memorized in the cell
fn label_dense_cell<const D: usize>(cell: &mut Cell<D>, params: & DBSCANParams, uf_str: &mut PartitionVec<CellIndex<D>>){
    cell.is_core = true;
    cell.core_info.uf_index = uf_str.len();
    let points : Vec<Point<D>> = cell.points.iter().map(|x| x.point).collect();
    cell.core_info.root = TreeStructure::build_structure(points, params);
    uf_str.push(cell.index);
    for mut s_point in &mut cell.points {
        s_point.is_core = true;
    }
}

/// Function to decide if a cell with less than 'MinPts' points inside is a core cell. If it is a core cell
/// then all the core points inside are labeled as such and the cell is added to the union-find structure 'uf_str' and its index
/// inside the structure is memorized in the cell. An approximate range counting structure is then built on the core points and 
/// memorized in the cell
fn label_sparse_cell<const D: usize>(cells_c: &CellTable<D>,curr_cell: &mut Cell<D>, params: &DBSCANParams, uf_str: &mut PartitionVec<CellIndex<D>>){
    let len = curr_cell.points.len();
    let mut points : Vec<Point<D>> = Vec::with_capacity(curr_cell.points.len());
    for mut s_point in &mut curr_cell.points {
        let mut tot_pts = len;
        for n_index in &curr_cell.neighbour_cell_indexes {
            if !is_same_index(&curr_cell.index, n_index) {
                // By using the r-tree fo populate the neighbours indexes I can be sure to get
                // `Some(neighbour)` from the `get` call.
                let neighbour = cells_c.get(n_index).unwrap();
                tot_pts += points_in_range(&s_point.point, neighbour, params.epsilon);
            }
            if tot_pts >= params.min_pts {
                break;
            }
        }
        if tot_pts >= params.min_pts {
            s_point.is_core = true;
            curr_cell.is_core = true;
            points.push(s_point.point.clone());
        }
    }
    if curr_cell.is_core {
        curr_cell.core_info.uf_index = uf_str.len();
        uf_str.push(curr_cell.index.clone());      
        curr_cell.core_info.root = TreeStructure::build_structure(points, params);
    }
}


/// Function that makes all the possible 'union' operations on the union-find structure `part_vec` on cells that have core points close enough
/// to create an arc between them. At the end of this function `part_vec` has as many sets inside as the number of approximate clusters and all 
/// cells in the same set contain all and only the core points that belong to the same cluster.
pub fn compute_adjacency_lists<const D: usize>(cells:  &mut CellTable<D>, params: &DBSCANParams, part_vec: &mut PartitionVec<CellIndex<D>>){
    for cell in cells.values().filter(|c| c.is_core) {
        for n_index in &cell.neighbour_cell_indexes {
            // By using the r-tree fo populate the neighbours indexes I can be sure to get
            // `Some(neighbour)` from the `get` call.
            let neighbour = cells.get(n_index).unwrap();
            if neighbour.is_core {
                if part_vec.same_set(cell.core_info.uf_index, neighbour.core_info.uf_index){
                    continue;
                }
                for point in cell.points.iter().filter(|p| p.is_core) {
                    if neighbour.core_info.root.approximate_range_counting_root(&point.point, params) != 0 {
                        part_vec.union(cell.core_info.uf_index, neighbour.core_info.uf_index);
                        break;
                    }  
                }
            }
            
        }
    }
}


#[cfg(test)]
mod tests;