/*
    Binary to make a table of fish-friendly numbers
    (as a function of m and n)
*/

use fish_friendly::count_friendly_grids;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(help = "Up to size (sum of rows + cols)")]
    upto: usize,
}

#[allow(clippy::needless_range_loop)]
pub fn main() {
    let Args { upto } = Args::parse();

    let mut friendly_table =
        vec![vec![None; upto]; upto];
    let mut total_table =
        vec![vec![None; upto]; upto];
    for size in 1..upto {
        for rows in 1..size {
            let cols = size - rows;
            println!("=== {} x {} grids ===", rows, cols);
            let (friendly, total) = count_friendly_grids(rows, cols);
            friendly_table[rows][cols] = Some(friendly);
            total_table[rows][cols] = Some(total);
        }
    }

    println!("=== Results ===");
    println!("Friendly:");
    for row in 1..upto {
        for col in 1..upto {
            let s = friendly_table[row][col].map(|x| format!("{:10}", x)).unwrap_or("".to_string());
            print!("{}", s);
        }
        println!();
    }
    println!("Total:");
    for row in 1..upto {
        for col in 1..upto {
            let s = total_table[row][col].map(|x| format!("{:10}", x)).unwrap_or("".to_string());
            print!("{}", s);
        }
        println!();
    }
}
