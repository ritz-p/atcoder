use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars;h]
    };
    let mut start = (0, 0);
    let mut goal = (0, 0);
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == 'S' {
                start = (i, j);
            }
            if s[i][j] == 'G' {
                goal = (i, j);
            }
        }
    }
    let mut visited = vec![vec![vec![false; 2]; w]; h];
    let mut queue = VecDeque::new();
    queue.push_back((start, true, 0));
    queue.push_back((start, false, 0));
    visited[start.0][start.1][1] = true;
    visited[start.0][start.1][0] = true;
    while let Some(((y, x), flag, current)) = queue.pop_front() {
        if (y, x) == goal {
            println!("{}", current);
            return;
        }
        if flag {
            if y + 1 < h {
                let next = (y + 1, x);
                if !visited[next.0][next.1][0] && s[next.0][next.1] != '#' {
                    visited[next.0][next.1][0] = true;
                    queue.push_back((next, !flag, current + 1));
                }
            }
            if y > 0 {
                let next = (y - 1, x);
                if !visited[next.0][next.1][0] && s[next.0][next.1] != '#' {
                    visited[next.0][next.1][0] = true;
                    queue.push_back((next, !flag, current + 1));
                }
            }
        } else {
            if x + 1 < w {
                let next = (y, x + 1);
                if !visited[next.0][next.1][1] && s[next.0][next.1] != '#' {
                    visited[next.0][next.1][1] = true;
                    queue.push_back((next, !flag, current + 1));
                }
            }
            if x > 0 {
                let next = (y, x - 1);
                if !visited[next.0][next.1][1] && s[next.0][next.1] != '#' {
                    visited[next.0][next.1][1] = true;
                    queue.push_back((next, !flag, current + 1));
                }
            }
        }
    }
    println!("-1");
}
