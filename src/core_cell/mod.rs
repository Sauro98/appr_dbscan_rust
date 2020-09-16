use crate::cell::*;
use crate::utils::*;
use crate::tree_structure::TreeStructure;
use partitions::PartitionVec;


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

fn is_same_index<const D: usize>(i1: &CellIndex<D>, i2: &CellIndex<D>) -> bool {
    for i in 0..D {
        if i1[i] != i2[i] {
            return false;
        }
    }
    true
}

pub fn label_points<const D: usize>(cells: &mut CellTable<D>, params: &DBSCANParams) -> PartitionVec<CellIndex<D>> {
    //nella struttura union find si andranno ad inserire le celle di core trovate, che sicuramente sono al massimo nello stesso numero delle 
    //celle non di core
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


fn label_sparse_cell<const D: usize>(cells_c: &CellTable<D>,curr_cell: &mut Cell<D>, params: &DBSCANParams, uf_str: &mut PartitionVec<CellIndex<D>>){
    let len = curr_cell.points.len();
    let mut points : Vec<Point<D>> = Vec::with_capacity(curr_cell.points.len());
    for mut s_point in &mut curr_cell.points {
        let mut tot_pts = len;
        for n_index in &curr_cell.neighbour_cell_indexes {
            if !is_same_index(&curr_cell.index, n_index) {
                //con l'r-tree faccio il calcolo dei neighbour solo sulle celle effettivamente presenti, quindi quando qui
                //faccio il get so gia' che mi restituira' Some(neighbour)
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

pub fn compute_adjacency_lists<const D: usize>(cells:  &mut CellTable<D>, params: &DBSCANParams, part_vec: &mut PartitionVec<CellIndex<D>>){
    for cell in cells.values().filter(|c| c.is_core) {
        for n_index in &cell.neighbour_cell_indexes {
            //con l'r-tree faccio il calcolo dei neighbour solo sulle celle effettivamente presenti, quindi quando qui
            //faccio il get so gia' che mi restituira' Some(neighbour)
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