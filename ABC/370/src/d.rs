use proconio::input;
use std::collections::{BTreeSet, HashSet};
fn main() {
    input! {
        h: usize,
        w: usize,
        q: usize,
        rc: [(usize,usize);q]
    };
    let mut row_walls: Vec<BTreeSet<usize>> = vec![(1..=w).collect(); h + 1];
    let mut col_walls: Vec<BTreeSet<usize>> = vec![(1..=h).collect(); w + 1];
    let mut walls = HashSet::new();
    for (r, c) in rc {
        if walls.contains(&(r, c)) {
            if let Some(&nearest_row) = row_walls[r].range(c..).next() {
                walls.insert((r, nearest_row));
                row_walls[r].remove(&nearest_row);
            }
            if let Some(&nearest_row_up) = row_walls[r].range(..c).next_back() {
                walls.insert((r, nearest_row_up));
                row_walls[r].remove(&nearest_row_up);
            }
            if let Some(&nearest_col) = col_walls[c].range(r..).next() {
                walls.insert((nearest_col, c));
                col_walls[c].remove(&nearest_col);
            }
            if let Some(&nearest_col_up) = col_walls[c].range(..r).next_back() {
                walls.insert((nearest_col_up, c));
                col_walls[c].remove(&nearest_col_up);
            }
        } else {
            walls.insert((r, c));
            row_walls[r].remove(&c);
            col_walls[c].remove(&r);
        }
    }
    println!("{:?}", row_walls);
    println!("{:?}", col_walls);
    println!("{:?}", walls);
    println!("{}", h * w - walls.len());
}
