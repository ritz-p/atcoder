use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
    };
    let mut v = vec![];
    for _i in 0..n {
        input! {
            l: usize,
            a: [usize;l]
        };
        v.push(a);
    }
    let mut res = 0;
    dfs(&v, 0, 1, n, x, &mut res);
    println!("{}", res);
}

fn dfs(v: &Vec<Vec<usize>>, current: usize, total: usize, n: usize, x: usize, res: &mut usize) {
    if current == n {
        if total == x {
            *res += 1;
        }
        return;
    }
    for e in &v[current] {
        if total > x / e {
            continue;
        }
        dfs(v, current + 1, total * e, n, x, res);
    }
}
