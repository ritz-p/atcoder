use std::collections::VecDeque;

use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        n: isize,
        m: isize,
    };
    let mut pos = vec![];
    for i in 0..=n {
        for j in 0..=n {
            if i * i + j * j == m {
                pos.push((i, j));
                pos.push((-i, j));
                pos.push((i, -j));
                pos.push((-i, -j));
            }
        }
    }
    let mut grid = vec![vec![-1; n as usize]; n as usize];
    grid[0][0] = 0;
    let mut queue = VecDeque::new();
    queue.push_back((0, 0));
    while let Some((y, x)) = queue.pop_front() {
        for (i, j) in &pos {
            let ni = y + i;
            let nj = x + j;
            if ni < n && nj < n && ni >= 0 && nj >= 0 {
                if grid[ni as usize][nj as usize] == -1 {
                    grid[ni as usize][nj as usize] = grid[y as usize][x as usize] + 1;
                    queue.push_back((ni, nj));
                }
            }
        }
    }
    for i in 0..n {
        println!("{}", grid[i as usize].iter().join(" "));
    }
}
