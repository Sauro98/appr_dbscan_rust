use std::fs::File;
use std::io::{Write};
use std::io::{self, BufRead};
use std::path::{Path};
use crate::utils::{DBSCANParams, Point, VectorDBSCANResult /*array_res_to_vector_res*/};

const PALETTE_ARR : [[u8; 3];64] = [
    [0, 0, 0],
    [1, 0, 103],
    [213, 255, 0],
    [255, 0, 86],
    [158, 0, 142],
    [14, 76, 161],
    [255, 229, 2],
    [0, 95, 57],
    [0, 255, 0],
    [149, 0, 58],
    [255, 147, 126],
    [164, 36, 0],
    [0, 21, 68],
    [145, 208, 203],
    [98, 14, 0],
    [107, 104, 130],
    [0, 0, 255],
    [0, 125, 181],
    [106, 130, 108],
    [0, 174, 126],
    [194, 140, 159],
    [190, 153, 112],
    [0, 143, 156],
    [95, 173, 78],
    [255, 0, 0],
    [255, 0, 246],
    [255, 2, 157],
    [104, 61, 59],
    [255, 116, 163],
    [150, 138, 232],
    [152, 255, 82],
    [167, 87, 64],
    [1, 255, 254],
    [255, 238, 232],
    [254, 137, 0],
    [189, 198, 255],
    [1, 208, 255],
    [187, 136, 0],
    [117, 68, 177],
    [165, 255, 210],
    [255, 166, 254],
    [119, 77, 0],
    [122, 71, 130],
    [38, 52, 0],
    [0, 71, 84],
    [67, 0, 44],
    [181, 0, 255],
    [255, 177, 103],
    [255, 219, 102],
    [144, 251, 146],
    [126, 45, 210],
    [189, 211, 147],
    [229, 111, 254],
    [222, 255, 116],
    [0, 255, 120],
    [0, 155, 255],
    [0, 100, 1],
    [0, 118, 255],
    [133, 169, 0],
    [0, 185, 23],
    [120, 130, 49],
    [0, 255, 198],
    [255, 110, 65],
    [232, 94, 190],
];

const OFFSET : u32 = 54;
const HEADER_SIZE : u32 = 40;
const PLANES: u16 = 1;
const BITS_PER_PIXEL : u16 = 24;
const COMPRESSION_METHOD : u32 = 0;
const H_RES : u32 = 0;
const W_RES : u32 = 0;
const COLORS_COUNT: u32 = 0;
const IMPORTANT_COLORS: u32 = 0;


fn read_lines<P>(filename: &P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file_res = File::open(filename);
    match file_res {
        Ok(file) => {
            Ok(io::BufReader::new(file).lines())
        },
        Err(e) => {
            println!("An error has occourred during file opening: {:?}",e);
            Err(e)
        }
    }
}

/// Reads the number of points and their dimensionality contained in a data file.
/// Points must be one for each row and their coordinates must be separated by whitespace.
/// 
/// ## Example:
/// ```text
/// 0.0 0.1 1.0
/// 1.0 2.0 1.5
/// ...
/// ```
pub fn params_from_file<P>(file_name: &P) -> DBSCANParams 
where P: AsRef<Path>, {
    let mut dim = 0;
    let mut card = 0;
    match read_lines(file_name){
        Ok(mut lines) => {
            let line : String = lines.next().unwrap().unwrap_or("".to_string());
            dim = line.split_whitespace().count();
            //aggiungo 1 per aver letto la prima riga
            card = 1;
            for line_opt in lines {
                let line = line_opt.unwrap_or("".to_string());
                if !line.trim().is_empty() {
                    card += 1;
                }
            }
        },
        Err(_e) => {
        }
    };

    DBSCANParams {
        dimensionality: dim as u32,
        cardinality: card,
        epsilon: 0.0,
        rho: 0.0,
        min_pts:0
    }
}

/// Reads `params.cardinality` points from a data file assuming that all of them have `D` components, panicking otherwise.
/// The rsult is stored as a vector of arrays of fixed length `D`.
pub fn read_points_from_file<P,const D: usize>(file_name: &P, params: &DBSCANParams) -> Vec<Point<D>>
where P: AsRef<Path>, {
    let mut points :  Vec<Point<D>> = Vec::with_capacity(params.cardinality);
    match read_lines(file_name) {
        Ok(lines) => {
            for line_opt in lines {
                let mut point : Point<D> = [0.0;D];
                let line = line_opt.unwrap_or("".to_string());
                if line.trim().is_empty() {
                    continue;
                }
                let mut p_i = 0;
                for val in line.split_whitespace() {
                    match val.parse() {
                        Ok(converted) => {
                            if p_i >= D {
                                panic!("This point does not have {} components: {}",D,line);
                            }
                            point[p_i] = converted;
                        },
                        Err(e) => {
                            println!("An error occourred while reading a point: check your input file. {:?}",e);
                        }
                    }
                    p_i += 1;
                }
                points.push(point);
            }
        },
        Err(_e) => {}
    }
    if points.len() != params.cardinality {
        panic!("Expected {} points in input file but {} were found", params.cardinality, points.len());
    }
    points
}

/// Same as `write_to_bmp_vec` but takes in input a DBSCANResult where each point is a fixed length array.
//TODO: decide what to do
/*pub fn write_to_bmp<P, const D: usize>(file_name: &P,res: &DBSCANResult<D>)
where P: AsRef<Path>, {
    write_to_bmp_vec(file_name, &array_res_to_vector_res(res.clone()), D);
}*/


/// Writes the clusterized result to a bmp image using high contrast colors for the different clusters. 
/// This assumes that the clusterized data comes from a 24 bit BMP image and writes `file_name` as a 24 bit BMP image.
/// The clusterized points must have 5 coordinates and they must be in this order to have a valid result:
/// row column B G R
/// 
/// # Parameters
/// * `file_name`: the path to the file where the bmp will be written;
/// * res: the result of the approximate DBSCAN algorithm where each point is represented as a vector. If this is not your
///   type of result check `write_to_bmp`;
/// * dimensionality: the number of components of the points in res.
///
pub fn write_to_bmp_vec<P>(file_name: &P,res: &VectorDBSCANResult, dimensionality: usize)
where P: AsRef<Path>, {
    if dimensionality != 5 {
        println!("Points are required to have 5 dimensionalities to be printed to bmp");
        return;
    }
    let mut gp_file = match File::create(file_name) {
        Err(why) => panic!("couldn't create {}:", why),
        Ok(file) => file,
    };
    let height = res.iter().map(|x| x.iter().map(|x| x[0] as i64).max().unwrap_or(0)).max().unwrap() + 1;
    let width = res.iter().map(|x| x.iter().map(|x| x[1] as i64).max().unwrap_or(0)).max().unwrap() + 1;
    println!("w{} h {}",width, height);
    let tot_pts : usize = (width * height) as usize;
    let map_size = 3 * tot_pts;
    let mut tot_size = OFFSET + map_size as u32;
    let padding = (width % 4) as u32;
    if padding != 0 {
        println!("warning: BMP needs padding");
        tot_size += padding  * height as u32 * 3;
    }

    // first insert into all_points all noise points adding an additional coordinate to represent their cluster number (0)
    let mut all_points : Vec<Vec<i64>>= {
        res[0].iter().map(|x| {let mut y : Vec<i64> = x.iter().map(|x| *x as i64).collect(); y.push(0); y}).collect()
    };
    // then append all the points from the oher clusters, adding a coordinate to represent their cluster number
    for i in 1..res.len() {
        all_points.append(&mut ({
                res[i].iter().map(|x| {let mut y: Vec<i64> = x.iter().map(|x| *x as i64).collect(); y.push(i as i64); y}).collect()
            }
        ));
    } 
    all_points.sort_by(|a,b| a[1].cmp(&b[1]));
    all_points.sort_by(|a,b| a[0].cmp(&b[0]));
    all_points.dedup_by(|a,b| a[0] == b[0] && a[1] == b[1]);

    //bitmap header
    gp_file.write_all(&[0x42,0x4D]).unwrap();
    //header
    gp_file.write_all(&(tot_size as u32).to_le_bytes()).unwrap();
    gp_file.write_all(&[0;4]).unwrap();
    gp_file.write_all(&(OFFSET).to_le_bytes()).unwrap();
    //dib header
    gp_file.write_all(&(HEADER_SIZE).to_le_bytes()).unwrap();
    gp_file.write_all(&(width as u32).to_le_bytes()).unwrap();
    gp_file.write_all(&(height as u32).to_le_bytes()).unwrap();
    gp_file.write_all(&(PLANES).to_le_bytes()).unwrap();
    gp_file.write_all(&(BITS_PER_PIXEL).to_le_bytes()).unwrap();
    gp_file.write_all(&(COMPRESSION_METHOD).to_le_bytes()).unwrap();
    gp_file.write_all(&(map_size as u32).to_le_bytes()).unwrap();
    gp_file.write_all(&(H_RES).to_le_bytes()).unwrap();
    gp_file.write_all(&(W_RES).to_le_bytes()).unwrap();
    gp_file.write_all(&(COLORS_COUNT).to_le_bytes()).unwrap();
    gp_file.write_all(&(IMPORTANT_COLORS).to_le_bytes()).unwrap();

    let mut col_counter = 0;
    for point in all_points {
        // coordinate at index dimensionality is the cluster index
        let color = PALETTE_ARR[(point[dimensionality] as usize % PALETTE_ARR.len()) as usize];
        gp_file.write_all(&[color[2], color[1], color[0]]).unwrap();
        col_counter += 1;
        if col_counter == width {
            for _i in 0..padding {
                gp_file.write_all(&[0]).unwrap();
            }
            col_counter = 0;
        }
    }

}


#[cfg(test)]
mod tests;