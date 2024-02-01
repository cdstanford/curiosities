/*
    Binary to randomly generate grids & estimate
    the probability
*/

use fish_friendly::Grid;

use clap::Parser;
use std::io::{self, Write};

// Number of steps to display progress
const PROGRESS_STEPS: usize = 10;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(help = "Number of rows")]
    rows: usize,
    #[arg(help = "Number of columns")]
    cols: usize,
    #[arg(short, long, help = "Number of iterations", default_value = "1000000")]
    num_iters: usize,
}

fn main() {
    let args = Args::parse();

    if args.num_iters % PROGRESS_STEPS != 0 {
        eprintln!("The number of iterations should be a multiple of {}", PROGRESS_STEPS);
        std::process::exit(1);
    }
    let progress_step = args.num_iters / 10;

    println!("Running {} iterations for {} x {} grids", args.num_iters, args.rows, args.cols);
    let mut friendly: usize = 0;
    for epoch in 0..(args.num_iters / progress_step) {
        print!(
            "Running iterations {}-{}...",
            epoch * progress_step,
            (epoch + 1) * progress_step - 1,
        );
        io::stdout().flush().unwrap();

        let mut new_friendly: usize = 0;
        for _ in 0..progress_step {
            let grid = Grid::new_random(args.rows, args.cols);
            if grid.fish_friendly() {
                new_friendly += 1;
            }
        }
        println!(" {} friendly", new_friendly);
        friendly += new_friendly;
    }

    println!("=== Results for {} x {} grids ===", args.rows, args.cols);
    println!(
        "The fish can swim across in {}/{} cases ({:.3}%).",
        friendly,
        args.num_iters,
        (friendly as f64) * 100.0 / (args.num_iters as f64),
    );
}
