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

use utils::{Point, DBSCANParams};
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
    params.epsilon = epsilon;
    params.rho = rho;
    params.min_pts = min_pts;
    println!("Epsilon: {}, Rho: {}, MinPts: {}",epsilon, rho, min_pts);
    println!("Dim: {}, n: {}",params.dimensionality, params.cardinality);
    do_dbscan(&params, file_name);
}

fn do_dbscan(params: &DBSCANParams, file_name: &str){
    match params.dimensionality {
        0 => println!("Errore nella lettura del file di dati"),
        1 => do_dbscan_d::<1>(params, file_name),
        2 => do_dbscan_d::<2>(params, file_name),
        3 => do_dbscan_d::<3>(params, file_name),
        4 => do_dbscan_d::<4>(params, file_name),
        5 => do_dbscan_d::<5>(params, file_name),
        6 => do_dbscan_d::<6>(params, file_name),
        7 => do_dbscan_d::<7>(params, file_name),
        _ => println!("Non sono supportate dimensionalita' oltre la settima")
    }
}

fn do_dbscan_d<const D: usize>(params: &DBSCANParams, file_name: &str) {
    let points: Vec<Point<D>> = read_points_from_file(file_name, &params);
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

