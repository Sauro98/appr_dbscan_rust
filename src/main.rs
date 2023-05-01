extern crate appr_dbscan;
use appr_dbscan::do_appr_dbscan_auto_dimensionality_file;
use appr_dbscan::data_io::{params_from_file, write_to_bmp_vec};
use std::env;
use std::process;



const MIN_ARGS_NUM : usize = 5;
const MAX_ARGS_NUM : usize = 6;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != MIN_ARGS_NUM && args.len() != MAX_ARGS_NUM {
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


    let (res, dimensionality) = do_appr_dbscan_auto_dimensionality_file(file_name, epsilon, rho, min_pts);
    if print_bitmap {
        write_to_bmp_vec(&"./gp_srcs/out.bmp",&res, dimensionality);
    }
    println!("Found {} clusters and {} noise points", res.len() -1 ,res[0].len());
}

fn print_help(){
    println!("Utilizzo:");
    println!(" - appr_dbscan_rust data_file epsilon rho min_pts <print_bitmap>");
    println!(" - data_file deve essere un file contenente punti stampati uno per riga e con le coordinate separate da uno spazio");
    println!(" - epsilon e rho devono essere numeri decimali positivi non nulli");
    println!(" - min_pts deve essere un numero intero positivo non nullo");
    println!(" - print_bitmap e' opzionale e se presente deve essere un valore booleano. Di default vale \"false\"");
    println!("Se uno tra 'print_bitmap' e 'compare_results' deve essere messo a true allora vanno specificati entrambi.");
}

fn parse_float(arg: &str, name: &str) -> f64 {
    let float : f64 = arg.parse().unwrap_or(0.0);
    if float <= 0.0 {
        println!("Il valore inserito per {:?} non e' valido : {}", name, arg);
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

