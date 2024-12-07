use proconio::{input, marker::Chars};
use std::collections::VecDeque;

fn main() {
    input! {
        h: usize,
        w: usize,
        d: usize,
        s: [Chars;h]
    };
    let mut res = 0;
    let mut flag = vec![vec![false; w]; h];
    let mut queue = VecDeque::new();

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == 'H' {
                queue.push_back((i, j, 0));
                flag[i][j] = true;
            }
        }
    }
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    while let Some((x, y, count)) = queue.pop_front() {
        if count >= d {
            continue;
        }
        for (dx, dy) in directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx >= 0
                && ny >= 0
                && (nx as usize) < h
                && (ny as usize) < w
                && s[nx as usize][ny as usize] == '.'
                && !flag[nx as usize][ny as usize]
            {
                flag[nx as usize][ny as usize] = true;
                queue.push_back((nx as usize, ny as usize, count + 1));
            }
        }
    }
    for i in 0..h {
        for j in 0..w {
            if flag[i][j] {
                res += 1;
            }
        }
    }

    println!("{}", res);
}
