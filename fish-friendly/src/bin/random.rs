/*
    Binary to randomly generate grids & estimate
    the probability
*/

use fish_friendly::Grid;

use std::io::{self, Write};

// Number of iterations to run
const NUM_ITERS: usize = 1000000;
// Number of steps to display progress
// (Should be a divisor of NUM_ITERS)
const PROGRESS_STEP: usize = 100000;

const ROWS: usize = 10;
const COLS: usize = 10;

fn main() {
    println!("Running {} iterations for {} x {} grids", NUM_ITERS, ROWS, COLS);
    let mut friendly: usize = 0;
    for epoch in 0..(NUM_ITERS / PROGRESS_STEP) {
        print!(
            "Running iterations {}-{}...",
            epoch * PROGRESS_STEP,
            (epoch + 1) * PROGRESS_STEP - 1,
        );
        io::stdout().flush().unwrap();

        let mut new_friendly: usize = 0;
        for _ in 0..PROGRESS_STEP {
            let grid = Grid::new_random(ROWS, COLS);
            if grid.fish_friendly() {
                new_friendly += 1;
            }
        }
        println!(" {} friendly", new_friendly);
        friendly += new_friendly;
    }

    println!("=== Results for {} x {} grids ===", ROWS, COLS);
    println!(
        "The fish can swim across in {}/{} cases ({:.3}%).",
        friendly,
        NUM_ITERS,
        (friendly as f64) * 100.0 / (NUM_ITERS as f64),
    );
}
