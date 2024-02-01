/*
    Binary to make a table of fish-friendly numbers
    (as a function of m and n)
*/

use fish_friendly::count_friendly_grids;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(help = "Up to number of rows")]
    rows_upto: usize,
    #[arg(help = "Up to number of columns")]
    cols_upto: usize,
}

#[allow(clippy::needless_range_loop)]
pub fn main() {
    let args = Args::parse();

    let mut friendly_table =
        vec![vec![0_u128; args.cols_upto + 1]; args.rows_upto + 1];
    let mut total_table =
        vec![vec![0_u128; args.cols_upto + 1]; args.rows_upto + 1];
    for rows in 1..=args.rows_upto {
        for cols in 1..=args.cols_upto {
            println!("=== {} x {} grids ===", rows, cols);
            let (friendly, total) = count_friendly_grids(rows, cols);
            friendly_table[rows][cols] = friendly;
            total_table[rows][cols] = total;
        }
    }

    println!("=== Results ===");
    println!("Friendly:");
    for row in 1..=args.rows_upto {
        for col in 1..=args.cols_upto {
            print!("{:10}", friendly_table[row][col]);
        }
        println!();
    }
    println!("Total:");
    for row in 1..=args.rows_upto {
        for col in 1..=args.cols_upto {
            print!("{:10}", total_table[row][col]);
        }
        println!();
    }
}
