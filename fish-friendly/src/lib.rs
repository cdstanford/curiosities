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

    pub fn cell(&self, i: usize, j: usize) -> bool {
        self.grid[i][j]
    }

    pub fn is_sink(&self, _i: usize, j: usize) -> bool {
        j == self.cols
    }

    pub fn sources(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (1..=self.rows).map(|i| (i, 1)).filter(|&(i, j)| self.cell(i, j))
    }

    pub fn adjacencies(
        &self,
        i: usize,
        j: usize,
    ) -> impl Iterator<Item = (usize, usize)> + '_ {
        iter::once((i - 1, j))
            .chain(iter::once((i + 1, j)))
            .chain(iter::once((i, j - 1)))
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
}
