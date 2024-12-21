use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        _w: usize,
        mut x: usize,
        mut y: usize,
        s: [Chars;h],
        t: Chars,
    };
    let mut set = HashSet::new();
    x -= 1;
    y -= 1;
    for c in t {
        match c {
            'U' => {
                if s[x - 1][y] == '.' {
                    x -= 1;
                } else if s[x - 1][y] == '@' {
                    x -= 1;
                    set.insert((x, y));
                }
            }
            'D' => {
                if s[x + 1][y] == '.' {
                    x += 1;
                } else if s[x + 1][y] == '@' {
                    x += 1;
                    set.insert((x, y));
                }
            }
            'L' => {
                if s[x][y - 1] == '.' {
                    y -= 1;
                } else if s[x][y - 1] == '@' {
                    y -= 1;
                    set.insert((x, y));
                }
            }
            'R' => {
                if s[x][y + 1] == '.' {
                    y += 1;
                } else if s[x][y + 1] == '@' {
                    y += 1;
                    set.insert((x, y));
                }
            }
            _ => {}
        }
    }
    println!("{} {} {}", x + 1, y + 1, set.len());
}
