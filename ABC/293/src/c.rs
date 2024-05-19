use std::collections::HashSet;

use proconio::input;
// hash 関係の操作苦手すぎる
fn main() {
    input! {
        h: usize,
        w: usize,
    }

    let mut grid = vec![vec![0; w + 2]; h + 2];
    for y in 1..=h {
        for x in 1..=w {
            input! {
                n: usize,
            }

            grid[x][y] = n;
        }
    }

    let mut stack = vec![];
    let mut set = HashSet::new();
    set.insert(grid[1][1]);
    stack.push((1, 1, set));

    let mut ans: usize = 0;

    while !stack.is_empty() {
        let (x, y, set) = stack.pop().unwrap();
        if x == w && y == h {
            ans += 1;
            continue;
        }

        if x + 1 <= w {
            let mut set = set.clone();
            if !set.contains(&grid[x + 1][y]) {
                set.insert(grid[x + 1][y]);
                stack.push((x + 1, y, set));
            }
        }

        if y + 1 <= h {
            let mut set = set.clone();
            if !set.contains(&grid[x][y + 1]) {
                set.insert(grid[x][y + 1]);
                stack.push((x, y + 1, set));
            }
        }
    }

    println!("{}", ans);
}
