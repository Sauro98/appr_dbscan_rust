use std::collections::HashMap;
use crate::utils::*;

#[derive(Clone)]
pub struct TreeStructure<const D: usize>{
    cell_index: CellIndex<D>,
    level: i32,
    cnt: usize,
    side_size: f64,
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

    pub fn build_structure(points: Vec<Point<D>>, params: &DBSCANParams) -> TreeStructure<D> {
        let base_side_size = params.epsilon/(params.dimensionality as  f64 ).sqrt();
        let mut levels_count: i32 = (1.0/params.rho).log(2.0).ceil() as i32;
        if levels_count < 1 {
            levels_count = 1;
        }
        
        //In questo programma viene creata una struttura ad albero per ogni cella e quindi si
        //sa gia' che tutti i punti della cella appartengono a root. Si procede dunque subito a dividere 
        //root in 2^d sottocelle.
        let mut root = TreeStructure::new(&[0;D], -1,base_side_size);
        root.cnt = points.len();
        
        for point in &points {
            let mut curr_side_size = base_side_size;
            let mut prev_child = &mut root;
            prev_child.cnt += 1;
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

    pub fn approximate_range_counting_root(&self, q: &Point<D>, params: &DBSCANParams) -> usize{
        let mut ans = 0;
        for child in self.children.values() {
            ans += child.approximate_range_counting(q, params);
        }
        ans
    }

    fn approximate_range_counting(&self, q: &Point<D>, params: &DBSCANParams) -> usize {
        let mut ans : usize = 0;
        let mut levels_count: i32 = (1.0/params.rho).log(2.0).ceil() as i32;
        if levels_count < 1 {
            levels_count = 1;
        }
        let intersection_type = determine_intersection(q, params, &self.cell_index, self.side_size);
        match intersection_type {
            IntersectionType::Disjoint => {},
            IntersectionType::FullyCovered => {
                ans += self.cnt;
            },
            IntersectionType::Intersecting => {
                if self.level < levels_count - 1 {
                    for child in self.children.values() {
                        ans += child.approximate_range_counting(q, params);
                    }
                } else {
                    ans += self.cnt;
                }
            }
        }
        ans
    }
}


#[cfg(test)]
mod tests;