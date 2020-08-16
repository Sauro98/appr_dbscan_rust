DEL /F /S ".\gp_srcs\"
cargo run --release %1 %2 %3 %4
gnuplot -p plot.gp
