use proconio::{
    input,
    marker::{Chars, Usize1},
};
use std::collections::VecDeque;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars;h],
        n: usize,
        rce: [(Usize1, Usize1, usize); n],
    }

    let mut start = (usize::MAX, usize::MAX);

    for i in 0..h {
        for j in 0..w {
            if a[i][j] == 'S' {
                start = (i, j);
            }
        }
    }

    let mut med = vec![vec![0; w]; h];
    for (r, c, e) in rce {
        med[r][c] = e;
    }
    if med[start.0][start.1] == 0 {
        println!("No");
        return;
    }

    let mut grid = vec![vec![0; w]; h];
    grid[start.0][start.1] = med[start.0][start.1];

    let move_dir = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut queue = VecDeque::new();
    queue.push_back(start);

    while let Some((i, j)) = queue.pop_front() {
        for (di, dj) in move_dir {
            let di = i as i32 + di;
            let dj = j as i32 + dj;
            if di < 0 || di >= h as i32 || dj < 0 || dj >= w as i32 {
                continue;
            }
            let di = di as usize;
            let dj = dj as usize;
            if a[di][dj] == 'T' {
                println!("Yes");
                return;
            }

            if grid[di][dj] >= grid[i][j] - 1 {
                if grid[i][j] - 1 != 0 {
                    continue;
                }
            }

            if grid[i][j] - 1 == 0 && med[di][dj] == 0 {
                continue;
            }

            grid[di][dj] = grid[i][j] - 1;

            if med[di][dj] > grid[di][dj] {
                grid[di][dj] = med[di][dj];
            }

            if a[di][dj] == '.' {
                queue.push_back((di, dj));
            }
        }
    }
    println!("No");
}
