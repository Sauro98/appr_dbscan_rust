use std::collections::HashMap;
use crate::utils::*;

#[derive(Clone)]
/// Tree structure that divides the space in nested cells to perform approximate range counting
/// Each member of this structure is a node in the tree
pub struct TreeStructure<const D: usize>{
    /// The index of the cell represented by this node
    cell_index: CellIndex<D>,
    /// The size of the cell 
    side_size: f64,
    /// The depth inside the tree where this node lays
    level: i32,
    /// The number of points cointained in the cell
    cnt: usize,
    /// The collection of nested sub-cells (bounded by 2^D at max, with D constant)
    children: HashMap<CellIndex<D>, TreeStructure<D>>,
}

impl <const D: usize> TreeStructure<D> {
    pub fn new(cell_index: &CellIndex<D>, level: i32, side_size: f64) -> TreeStructure<D> {
        let structure = TreeStructure {
            cell_index: cell_index.clone(),
            level: level,
            cnt: 0,
            side_size: side_size,
            // mettere sempre 2^D come dimensione assicura che non ci siano riallocazione ma occupa 
            // troppo spazio. sembra che funzioni piu' velocemente senza dare una capacity
            children: HashMap::new()
        };
        structure
    }

    pub fn new_empty() -> TreeStructure<D>{
        TreeStructure{
            cell_index: [0;D],
            level: 0,
            cnt: 0,
            side_size: 0.0,
            children: HashMap::with_capacity(0)
        }
    }

    /// Generates a tree starting from the points given in input. To function correctly the points in input
    /// must be all and only the core points in a given cell of the approximated DBSCAN algorithm with side size
    /// equal to `epsilon/sqrt(D)`. This is assumed true during the construction.
    pub fn build_structure(points: Vec<Point<D>>, params: &DBSCANParams) -> TreeStructure<D> {
        let base_side_size = params.epsilon/(params.dimensionality as  f64 ).sqrt();
        let levels_count_f = 1.0 + (1.0/params.rho).log(2.0).ceil();
        let levels_count = if levels_count_f < 1.0 {
            1
        } else {
            levels_count_f as i32
        };
        // The approximated DBSCAN algorithm needs one instance of this structure for every core cell. 
        // This gives that all the points in input are contained in the cell of side size `epsilon/sqrt(D)`. 
        // All the points can then be added to the root and we proceed directly to divide the core cell in its sub-cells
        let mut root = TreeStructure::new(&get_base_cell_index(&points[0], params),0,base_side_size);
        root.cnt = points.len();
        
        for point in &points {
            let mut curr_side_size = base_side_size;
            let mut prev_child = &mut root;
            //il livello 0 è occupato dalla radice
            for i in 1..=levels_count {
                curr_side_size = curr_side_size / 2.0;
                let index_arr = get_cell_index(point, curr_side_size);
                let curr_child : &mut TreeStructure<D> =
                    prev_child.children.entry(index_arr.clone())
                    .or_insert(TreeStructure::new(&index_arr, i, curr_side_size));
                curr_child.cnt += 1;
                prev_child = curr_child;
            }
        }
        root
    }

    /// Performs the approximated range counting on the tree given the point in input. It stops as soon as the counting
    /// is non zero, so the result is not actually the exact count but rather 0 if there is no point in the tree
    /// in the vicinity of `q`, and a value that is less or equal to the number of points in the vicinity of `q` otherwise.
    /// The points in the vicinity are found for certain if they are at a distance less than equal to `epsilon` from `q` and 
    /// are excluded for certain if their distance from `q` is greater than `epsilon(1 + rho)`. All the points in between are 
    /// counted in an arbitrary way, depending on what is more efficient. 
    pub fn approximate_range_counting_root(&self, q: &Point<D>, params: &DBSCANParams) -> usize{
        self.approximate_range_counting(q,params)
    }

    fn approximate_range_counting(&self, q: &Point<D>, params: &DBSCANParams) -> usize {
        let mut ans : usize = 0;
        let levels_count_f = 1.0 + (1.0/params.rho).log(2.0).ceil();
        let levels_count = if levels_count_f < 1.0 {
            1
        } else {
            levels_count_f as i32
        };
        let intersection_type = determine_intersection(q, params, &self.cell_index, self.side_size);
        match intersection_type {
            IntersectionType::Disjoint => {},
            IntersectionType::FullyCovered => {
                ans += self.cnt;
            },
            IntersectionType::Intersecting => {
                if self.level < (levels_count - 1) {
                    for child in self.children.values() {
                        ans += child.approximate_range_counting(q, params);
                        /*if ans > 0 {
                            return ans;
                        }*/
                    }
                } else {
                    ans += self.cnt;
                }
            }
        }
        ans
    }

    

    /*fn print_tree_rec(&self) {
        println!("--- node ---");
        println!("> Level: {}",self.level);
        println!("> cell_index: {:?}",self.cell_index);
        println!("> cnt: {}",self.cnt);
        for child in self.children.values() {
            child.print_tree_rec();
        }
    }

    pub fn print_tree(&self){
        println!("----- TREE -----");
        self.print_tree_rec();
    }*/
}


#[cfg(test)]
mod tests;