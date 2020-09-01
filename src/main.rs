#![feature(min_const_generics)]
mod utils;
mod tree_structure;
mod cell;
mod core_cell;
mod cluster;
mod dbscan;
mod data_io;

use std::env;
use std::time::{Instant};
use std::process;
//use std::fs::File;
//use std::io::{Write};

use utils::{Point, DBSCANParams};
use data_io::*;
use dbscan::approximate_dbscan;

extern crate partitions;
extern crate rstar;

const MIN_ARGS_NUM : usize = 5;
const MAX_ARGS_NUM : usize = 6;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < MIN_ARGS_NUM && args.len() > MAX_ARGS_NUM {
        println!("Numero di parametri errato");
        print_help();
        return;
    }
    let file_name = &args[1];
    let epsilon = parse_float(&args[2], "epsilon");
    let rho = parse_float(&args[3], "rho");
    let min_pts = parse_usize(&args[4], "min_pts");
    let print_bitmap = if args.len() == MAX_ARGS_NUM {parse_bool(&args[5])} else {false};
    let mut params = params_from_file(file_name); 
    params.epsilon = epsilon;
    params.rho = rho;
    params.min_pts = min_pts;
    println!("Epsilon: {}, Rho: {}, MinPts: {}",epsilon, rho, min_pts);
    println!("Dim: {}, n: {}, Apprx_rdx: {}",params.dimensionality, params.cardinality, params.epsilon*(1_f64 +params.rho));
    do_dbscan(&params, file_name, print_bitmap);
}

fn do_dbscan(params: &DBSCANParams, file_name: &str, print_bitmap: bool){
    match params.dimensionality {
        0 => println!("Errore nella lettura del file di dati"),
        1 => do_dbscan_d::<1>(params, file_name, print_bitmap),
        2 => do_dbscan_d::<2>(params, file_name, print_bitmap),
        3 => do_dbscan_d::<3>(params, file_name, print_bitmap),
        4 => do_dbscan_d::<4>(params, file_name, print_bitmap),
        5 => do_dbscan_d::<5>(params, file_name, print_bitmap),
        6 => do_dbscan_d::<6>(params, file_name, print_bitmap),
        7 => do_dbscan_d::<7>(params, file_name, print_bitmap),
        _ => println!("Non sono supportate dimensionalita' oltre la settima")
    }
}

fn do_dbscan_d<const D: usize>(params: &DBSCANParams, file_name: &str, print_bitmap: bool) {
    let points: Vec<Point<D>> = read_points_from_file(file_name, &params);
    let now = Instant::now();
    let res = approximate_dbscan(points, &params);
    println!("Completed DBSCAN in {} milliseconds", now.elapsed().as_millis());
    if print_bitmap {
        write_to_bmp(&res);
    }
}

fn print_help(){
    println!("Utilizzo:");
    println!("appr_dbscan_rust data_file epsilon rho min_pts <print_bitmap>");
    println!("data_file deve essere un file contenente punti stampati uno per riga e con le coordinate separate da uno spazio");
    println!("epsilon e rho devono essere numeri decimali positivi non nulli");
    println!("min_pts deve essere un numero intero positivo non nullo");
    println!("print_bitmap e' opzionale e se presente deve essere un valore booleano. Di default vale \"false\"");
}

fn parse_float(arg: &str, name: &str) -> f64 {
    let float : f64 = arg.parse().unwrap_or(0.0);
    if float <= 0.0 {
        println!("Il valore inserito per {:?} non e' valido", name);
        process::exit(1);
    }
    float
}

fn parse_usize(arg: &str, name: &str) -> usize {
    let integer : usize = arg.parse().unwrap_or(0);
    if integer <= 0 {
        println!("Il valore inserito per {:?} non e' valido", name);
        process::exit(1);
    }
    integer
}

fn parse_bool(arg: &str) -> bool {
    let boolean : bool = arg.parse().unwrap_or(false);
    boolean
}

    /*let mut gp_file = match File::create("./plot.gp".to_string()) {
        Err(why) => panic!("couldn't create {}:", why),
        Ok(file) => file,
    };
    gp_file.write("set nokey \n plot".as_bytes()).unwrap();
    for i in 0..res.len() {
        let filename = "./gp_srcs/cl_".to_string() + &i.to_string()+ &".txt".to_string();
        let mut cluster_file = match File::create(&filename) {
            Err(why) => panic!("couldn't create {}:", why),
            Ok(file) => file,
        };
    
        for j in 0..res[i].len() {
            //TODO::D
            for k in 0..2{
                cluster_file.write(&res[i][j][k].to_string().as_bytes()).unwrap();
                cluster_file.write(&" ".as_bytes()).unwrap();
            }
            cluster_file.write(&"\n".as_bytes()).unwrap();
        }
        
        gp_file.write(("\"".to_string()+&filename+&"\" using 1:2 pt \"■\" lw 1 ps 0.001, \\".to_string()).as_bytes()).unwrap();
        gp_file.write(&"\n".as_bytes()).unwrap();
    }*/

