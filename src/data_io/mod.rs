use std::fs::File;
use std::io::{Write};
use std::io::{self, BufRead};
use std::path::{Path};
use crate::utils::{DBSCANParams, Point};
use crate::cluster::DBSCANResult;

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


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
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

pub fn params_from_file<P>(file_name: P) -> DBSCANParams 
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

pub fn read_points_from_file<P,const D: usize>(file_name: P, params: &DBSCANParams) -> Vec<Point<D>>
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
    points
}

pub fn write_to_bmp<P, const D: usize>(file_name: P,res: &DBSCANResult<D>)
where P: AsRef<Path>, {
    if D != 5 {
        println!("Per stampare su bitmap occore avere dati in 5 dimensioni");
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
    let mut all_points : Vec<Vec<i64>>= {
        res[0].iter().map(|x| {let mut y : Vec<i64> = x.to_vec().iter().map(|x| *x as i64).collect(); y.push(0); y}).collect()
    };
    for i in 1..res.len() {
        all_points.append(&mut ({
                res[i].iter().map(|x| {let mut y: Vec<i64> = x.to_vec().iter().map(|x| *x as i64).collect(); y.push(i as i64); y}).collect()
            }
        ));
    } 
    all_points.sort_by(|a,b| a[1].cmp(&b[1]));
    all_points.sort_by(|a,b| a[0].cmp(&b[0]));
    all_points.dedup_by(|a,b| a[0] == b[0] && a[1] == b[1]);
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
        let color = PALETTE_ARR[(point[D] as usize % PALETTE_ARR.len()) as usize];
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

/*pub fn compare_DBSCAN_results<P, const D: usize>(folder_path: P,res: &DBSCANResult<D>)
where P: AsRef<Path> {

}*/



#[cfg(test)]
mod tests;