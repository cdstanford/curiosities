/*
    Problem abstractions
*/

use rand::Rng;
use std::collections::HashSet;
use std::hash::Hash;
use std::iter;

/*
    Generic depth-first search
*/
pub fn dfs<T, Src, Succ, Succs, Snk>(
    sources: Src,
    get_succs: Succ,
    is_sink: Snk,
) -> bool
where
    T: Clone + Hash + Eq,
    Src: Iterator<Item = T>,
    Succ: Fn(&T) -> Succs,
    Succs: Iterator<Item = T>,
    Snk: Fn(&T) -> bool,
{
    let mut visited: HashSet<T> = HashSet::new();
    let mut to_visit: Vec<T> = sources.collect();
    while let Some(curr) = to_visit.pop() {
        if visited.contains(&curr) {
            continue;
        } else if is_sink(&curr) {
            return true;
        } else {
            visited.insert(curr.clone());
            for next in get_succs(&curr) {
                to_visit.push(next);
            }
        }
    }
    false
}

/*
    Grid struct
*/
pub struct Grid {
    rows: usize,
    cols: usize,
    grid: Vec<Vec<bool>>,
}

impl Grid {
    #[allow(clippy::needless_range_loop)]
    pub fn new_random(rows: usize, cols: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut grid = vec![vec![false; cols + 2]; rows + 2];
        for row in 1..=rows {
            for col in 1..=cols {
                grid[row][col] = rng.gen();
            }
        }
        Self { rows, cols, grid }
    }

    pub fn new_empty(rows: usize, cols: usize) -> Self {
        let grid = vec![vec![false; cols + 2]; rows + 2];
        Self { rows, cols, grid }
    }

    pub fn cell(&self, i: usize, j: usize) -> bool {
        self.grid[i][j]
    }

    pub fn sources(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (1..=self.rows).map(|i| (i, 1)).filter(|&(i, j)| self.cell(i, j))
    }

    pub fn is_sink(&self, _i: usize, j: usize) -> bool {
        j == self.cols
    }

    pub fn adjacencies(
        &self,
        i: usize,
        j: usize,
    ) -> impl Iterator<Item = (usize, usize)> + '_ {
        iter::once((i - 1, j))
            .chain(iter::once((i + 1, j)))
            .chain(iter::once((i, j - 1))) // note: remove this line to prevent "backwards" steps
            .chain(iter::once((i, j + 1)))
            .filter(|&(r, c)| self.cell(r, c))
    }

    // Check if the fish can get across swimming only on 'true' cells
    pub fn fish_friendly(&self) -> bool {
        dfs(
            self.sources(),
            |&(i, j)| self.adjacencies(i, j),
            |&(i, j)| self.is_sink(i, j),
        )
    }

    // Methods to iterate over all grids
    // return false in case of overflow

    fn rotate_cell(&mut self, i: usize, j: usize) -> bool {
        self.grid[i][j] = !self.grid[i][j];
        self.grid[i][j]
    }

    fn rotate_row(&mut self, i: usize) -> bool {
        (1..=self.cols).any(|j| self.rotate_cell(i, j))
    }

    pub fn rotate(&mut self) -> bool {
        (1..=self.rows).any(|i| self.rotate_row(i))
    }
}

const PROGRESS_STEPS: u128 = 10;

pub fn count_friendly_grids(rows: usize, cols: usize) -> (u128, u128) {
    let expect_total: u128 = 2_u128
        .checked_pow((rows * cols) as u32)
        .expect("The number of grids is too large to fit in a u128");
    let progress_step = expect_total / PROGRESS_STEPS;

    let mut total: u128 = 0;
    let mut friendly: u128 = 0;
    let mut grid = Grid::new_empty(rows, cols);
    loop {
        if grid.fish_friendly() {
            friendly += 1;
        }
        total += 1;
        if progress_step > 0 && total % progress_step == 0 {
            println!(
                "Progress: {:.0}%",
                (total as f64) * 100.0 / (expect_total as f64)
            );
        }
        if !grid.rotate() {
            break;
        }
    }
    debug_assert_eq!(total, expect_total);

    (friendly, total)
}
