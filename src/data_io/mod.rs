use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path};
use crate::utils::{DBSCANParams, Point};

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