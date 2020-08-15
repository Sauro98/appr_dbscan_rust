use std::collections::HashMap;
use crate::utils::*;

#[derive(Clone)]
pub struct TreeStructure{
    cell_index: Vec<i64>,
    level: i32,
    cnt: usize,
    children: HashMap<Vec<i64>, TreeStructure>,
}

impl TreeStructure {
    pub fn new(childrens_est: usize,cell_index: &Vec<i64>, level: i32) -> TreeStructure {
        let structure = TreeStructure {
            cell_index: cell_index.clone(),
            level: level,
            cnt: 0,
            children: HashMap::with_capacity(childrens_est)
        };
        structure
    }

    pub fn build_structure(points: &Vec<Vec<f64>>, params: &DBSCANParams) -> TreeStructure {
        let mut root = TreeStructure::new(points.len(), &Vec::with_capacity(0), -1);
        root.cnt = points.len();
        let max_children_count = 2_u64.pow(params.dimensionality);
        let mut levels_count: i32 = (1.0/params.rho).log(2.0).ceil() as i32;
        if levels_count < 1 {
            levels_count = 1;
        }
        for point in points {
            let mut curr_side_size = params.epsilon/(params.dimensionality as  f64 ).sqrt();
            let index_arr = get_cell_index(point, curr_side_size);
            let mut prev_child : &mut TreeStructure =
                root.children.entry(index_arr.clone())
                .or_insert(TreeStructure::new(max_children_count as usize, &index_arr, 0));
            prev_child.cnt += 1;
            for i in 1..levels_count {
                curr_side_size = curr_side_size / 2.0;
                let index_arr = get_cell_index(point, curr_side_size);
                let curr_child : &mut TreeStructure =
                    prev_child.children.entry(index_arr.clone())
                    .or_insert(TreeStructure::new(max_children_count as usize, &index_arr, i));
                curr_child.cnt += 1;
                prev_child = curr_child;
            }
        }
        root
    }

    pub fn approximate_range_counting_root(&self, q: &Vec<f64>, params: &DBSCANParams) -> usize{
        let mut ans = 0;
        for child in self.children.values() {
            ans += child.approximate_range_counting(q, params);
        }
        ans
    }

    fn approximate_range_counting(&self, q: &Vec<f64>, params: &DBSCANParams) -> usize {
        let mut ans : usize = 0;
        let mut levels_count: i32 = (1.0/params.rho).log(2.0).ceil() as i32;
        if levels_count < 1 {
            levels_count = 1;
        }
        let side_size = params.epsilon / (params.dimensionality as f64).sqrt();
        let side_size = side_size / 2_f64.powf(self.level as f64);
        let intersection_type = determine_intersection(q, params, &self.cell_index, side_size);
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