use proconio::input;
use std::collections::{BTreeSet, HashSet};
fn main() {
    input! {
        h: usize,
        w: usize,
        q: usize,
        rc: [(usize,usize);q]
    };
    let mut row: Vec<BTreeSet<usize>> = vec![(1..=w).collect(); h + 1];
    let mut col: Vec<BTreeSet<usize>> = vec![(1..=h).collect(); w + 1];
    let mut walls = HashSet::new();
    for (r, c) in rc {
        if walls.contains(&(r, c)) {
            if let Some(&right) = row[r].range(c + 1..).next() {
                walls.insert((r, right));
                row[r].remove(&right);
                col[right].remove(&r);
            }
            if let Some(&left) = row[r].range(..c).next_back() {
                walls.insert((r, left));
                row[r].remove(&left);
                col[left].remove(&r);
            }
            if let Some(&down) = col[c].range(r + 1..).next() {
                walls.insert((down, c));
                row[down].remove(&c);
                col[c].remove(&down);
            }
            if let Some(&up) = col[c].range(..r).next_back() {
                walls.insert((up, c));
                row[up].remove(&c);
                col[c].remove(&up);
            }
        } else {
            walls.insert((r, c));
            row[r].remove(&c);
            col[c].remove(&r);
        }
    }
    println!("{}", h * w - walls.len());
}
