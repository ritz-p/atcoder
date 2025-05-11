use std::collections::VecDeque;

use proconio::{input, marker::Chars};
fn main() {
    input! {
        h: usize,
        w: usize,
        mut s: [Chars;h]
    };
    let mut queue = VecDeque::new();
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == 'E' {
                queue.push_back((i, j));
            }
        }
    }

    while let Some((i, j)) = queue.pop_front() {
        if i > 0 {
            if s[i - 1][j] == '.' {
                s[i - 1][j] = 'v';
                queue.push_back((i - 1, j));
            }
        }
        if j > 0 {
            if s[i][j - 1] == '.' {
                s[i][j - 1] = '>';
                queue.push_back((i, j - 1));
            }
        }
        if i < h - 1 {
            if s[i + 1][j] == '.' {
                s[i + 1][j] = '^';
                queue.push_back((i + 1, j));
            }
        }
        if j < w - 1 {
            if s[i][j + 1] == '.' {
                s[i][j + 1] = '<';
                queue.push_back((i, j + 1));
            }
        }
    }

    for cs in s {
        println!("{}", cs.iter().collect::<String>());
    }
}
