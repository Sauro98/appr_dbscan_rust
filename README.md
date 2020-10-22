# appr_dbscan_rust
Rust implementation of the approximate version of DBSCAN introduced by Gan and Tao in this [paper](https://www.cse.cuhk.edu.hk/~taoyf/paper/tods17-dbscan.pdf)

# Recognized data files

Accepted data files should contain one data point per line and nothing else. Each line should contain the components of the point separated by whitespace. 

Each component of a point will be read and stored as a 64 bit floating point variable

### Example 

```text
1.0 1.1 0.5
2.3 3.4 6.2
...
```

# Usage

There are four main functions in this library that differ in the kind of input they accept:
  
### Warning

Each function below expects all points in the data file to have the same dimensionality and panics otherwise. 
  
## Approximated DBSCAN from data file with fixed dimensionality
 
If the dimensionality of each point is statically known (so not a result from another calculation) then this function can be used:

```rust
pub fn do_appr_dbscan_file<P, const D: usize>(
    filename: P, 
    epsilon: f64, 
    rho: f64, 
    min_pts: usize
) -> DBSCANResult<D> 
where
    P: AsRef<Path>, 
```

### Example

```rust
extern crate appr_dbscan;
use appr_dbscan::do_appr_dbscan_file;
use appr_dbscan::utils::DBSCANResult;
 
let res : DBSCANResult<2> = do_appr_dbscan_file("./datasets/out_test_1.txt", 0.3, 0.1, 10);
let clusters_count = res.len() - 1;
let noise_points_count = res[0].len();
```
 
## Approximated DBSCAN from data file with unknown dimensionality
 
If the dimensionality of the data points is not statically known (like if there is a loop going through multiple files with different dimensionalities) then this function can be used:

```rust
pub fn do_appr_dbscan_auto_dimensionality_file<P>(
    filename: P, 
    epsilon: f64, 
    rho: f64, 
    min_pts: usize
) -> (VectorDBSCANResult, usize) 
where
    P: AsRef<Path>, 
```

### Example 
 
```rust
extern crate appr_dbscan;
use appr_dbscan::do_appr_dbscan_auto_dimensionality_file;

let (res,dimensionality) = do_appr_dbscan_auto_dimensionality_file("./datasets/out_test_1.txt", 0.3, 0.1, 10);
println!("Points dimensionality: {}",dimensionality);
let clusters_count = res.len() - 1;
let noise_points_count = res[0].len();
```
 
## Approximated DBSCAN from vector of points of fixed dimensionality

If you have a vector of points of the type `Vec<[f64;D]>` then this function can be used:

```rust
pub fn do_appr_dbscan_points<const D: usize>(
    points: Vec<Point<D>>, 
    epsilon: f64, 
    rho: f64, 
    min_pts: usize
) -> DBSCANResult<D>
```

### Example

```rust
extern crate appr_dbscan;
use appr_dbscan::do_appr_dbscan_points;
use appr_dbscan::utils::DBSCANResult;
 
let points = vec![[0.0,0.0],[1.0,1.0],[0.0,1.0],[1.0,0.0],[2.0,1.0],[0.0,2.0],[2.0,1.0],[1.0,1.0]];
let res : DBSCANResult<2> = do_appr_dbscan_points(points, 0.3, 0.1, 10);
let clusters_count = res.len() - 1;
let noise_points_count = res[0].len();
```

## Approximated DBSCAN from vector of points of unknown dimensionality

If you have a vector of points of the type `Vec<Vec<f64>>` (in example if you are in a loop clustering different vectors) then this function can be used:

```rust
pub fn do_appr_dbscan_auto_dimensionality_points(
    points: Vec<VectorPoint>, 
    epsilon: f64, 
    rho: f64, 
    min_pts: usize
) -> (VectorDBSCANResult, usize)
```

### Example

```rust
extern crate appr_dbscan;
use appr_dbscan::do_appr_dbscan_auto_dimensionality_points;

let points = vec![vec![0.0,0.0],vec![1.0,1.0],vec![0.0,1.0],vec![1.0,0.0],vec![2.0,1.0],vec![0.0,2.0],vec![2.0,1.0],vec![1.0,1.0]];
let (res, dimensionality) = do_appr_dbscan_auto_dimensionality_points(points, 0.3, 0.1, 10);
println!("Points dimensionality: {}",dimensionality);
let clusters_count = res.len() - 1;
let noise_points_count = res[0].len();
```

