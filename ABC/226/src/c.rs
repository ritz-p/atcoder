use std::collections::HashSet;
use proconio::input;

fn main() {
    input! {
        n: usize
    };
    let mut av = vec![vec![];n];
    let mut tv = vec![];
    let mut kv = vec![];
    for i in 0..n{
        input!{
            t: usize,
            k: usize,
        };
        tv.push(t);
        kv.push(k);
        for _j in 0..k{
            input!{
                a: usize
            };
            av[i].push(a);
        }
    }
    let mut visited = HashSet::new();
    visited.insert(n-1);

    let mut res = 0;
    for i in (0..n).rev() {
        if visited.contains(&i) {
            res += tv[i];
            for j in 0..kv[i] {
                visited.insert(av[i][j] - 1);
            }
        }
    }
    println!("{}",res);
}
