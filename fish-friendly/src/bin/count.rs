/*
    Binary to count total # of grids
*/

use fish_friendly::count_friendly_grids;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(help = "Number of rows")]
    rows: usize,
    #[arg(help = "Number of columns")]
    cols: usize,
}

pub fn main() {
    let args = Args::parse();

    let (friendly, total) = count_friendly_grids(args.rows, args.cols);

    println!("=== Results for {} x {} grids ===", args.rows, args.cols);
    println!(
        "The fish can swim across in {} of {} cases ({:.3}%).",
        friendly,
        total,
        (friendly as f64) * 100.0 / (total as f64),
    );
}
