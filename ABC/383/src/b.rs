use std::collections::HashSet;

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        d: usize,
        s: [Chars;h]
    };
    let mut res = 0;
    let mut flag = vec![];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                flag.push((i, j));
            }
        }
    }
    for perm in flag.iter().combinations(2) {
        let mut set = HashSet::new();
        set.insert(*perm[0]);
        set.insert(*perm[1]);
        for i in 0..h {
            for j in 0..w {
                if s[i][j] == '.' {
                    if ((perm[0].0 as isize - i as isize).abs()
                        + (perm[0].1 as isize - j as isize).abs()) as usize
                        <= d
                    {
                        set.insert((i, j));
                    }
                    if ((perm[1].0 as isize - i as isize).abs()
                        + (perm[1].1 as isize - j as isize).abs()) as usize
                        <= d
                    {
                        set.insert((i, j));
                    }
                }
            }
        }
        res = res.max(set.len())
    }
    println!("{}", res);
}
