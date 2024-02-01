/*
    Binary to count total # of grids
*/

use fish_friendly::Grid;

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

    let expect_total: u128 = 2_u128.checked_pow((args.rows * args.cols) as u32).expect(
        "The number of grids is too large to fit in a u128"
    );
    let progress_step = expect_total / 10;

    let mut total: u128 = 0;
    let mut friendly: u128 = 0;
    let mut grid = Grid::new_empty(args.rows, args.cols);
    loop {
        if grid.fish_friendly() {
            friendly += 1;
        }
        total += 1;
        if total % progress_step == 0 {
            println!("Progress: {:.0}%", (total as f64) * 100.0 / (expect_total as f64));
        }
        if !grid.rotate() {
            break;
        }
    }
    debug_assert_eq!(total, expect_total);

    println!("=== Results for {} x {} grids ===", args.rows, args.cols);
    println!(
        "The fish can swim across in {} of {} cases ({:.3}%).",
        friendly,
        total,
        (friendly as f64) * 100.0 / (total as f64),
    );
}
