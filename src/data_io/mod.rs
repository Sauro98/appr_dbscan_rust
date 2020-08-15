use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path};
use crate::utils::DBSCANParams;

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
            dim = line.matches(' ').count();
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

pub fn read_points_from_file<P>(file_name: P, params: &DBSCANParams) -> Vec<Vec<f64>>
where P: AsRef<Path>, {
    let mut points :  Vec<Vec<f64>> = Vec::with_capacity(params.cardinality);
    match read_lines(file_name) {
        Ok(lines) => {
            for line_opt in lines {
                let mut point : Vec<f64> = Vec::with_capacity(params.dimensionality as usize);
                let line = line_opt.unwrap_or("".to_string());
                if line.trim().is_empty() {
                    continue;
                }
                for val in line.split(' ') {
                    if val.is_empty() {
                        continue;
                    }
                    match val.parse() {
                        Ok(converted) => {
                            point.push(converted);
                        },
                        Err(e) => {
                            println!("An error occourred while reading a point: check your input file. {:?}",e);
                        }
                    }
                }
                //println!("{:?}",point);
                points.push(point);
            }
        },
        Err(_e) => {}
    }
    points
}



#[cfg(test)]
mod tests;