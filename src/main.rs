mod utils;
mod tree_structure;
mod cell;
mod core_cell;
mod cluster;
mod dbscan;
mod data_io;

use std::env;
use std::time::{Duration, Instant};
use std::process;

use data_io::*;
use dbscan::approximate_dbscan;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        println!("Numero di parametri errato");
        print_help();
        return;
    }
    let file_name = &args[1];
    let epsilon = parse_float(&args[2], "epsilon");
    let rho = parse_float(&args[3], "rho");
    let min_pts = parse_usize(&args[4], "min_pts");
    let mut params = params_from_file(file_name);
    let points = read_points_from_file(file_name, &params);
    params.epsilon = epsilon;
    params.rho = rho;
    params.min_pts = min_pts;
    println!("Epsilon: {}, Rho: {}, MinPts: {}",epsilon, rho, min_pts);
    let now = Instant::now();
    let _res = approximate_dbscan(&points, &params);
    println!("In {} milliseconds", now.elapsed().as_millis());
}

fn print_help(){
    println!("Utilizzo:");
    println!("appr_dbscan_rust data_file epsilon rho min_pts");
    println!("data_file deve essere un file contenente punti stampati uno per riga e con le coordinate separate da uno spazio");
    println!("epsilon e rho devono essere numeri decimali positivi non nulli");
    println!("min_pts deve essere un numero intero positivo non nullo");
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

