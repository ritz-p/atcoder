use proconio::input;
use std::collections::HashSet;
fn main() {
    input! {
        n: usize,
        a: [usize;n]
    };
    let mut set = HashSet::new();
    let mut part = vec![];
    dfs(&a, &mut part, 0, &mut set);
    println!("{}", set.len());
}

fn dfs(a: &Vec<usize>, part: &mut Vec<usize>, index: usize, set: &mut HashSet<usize>) {
    let n = a.len();
    if index == n {
        let mut xor = 0;
        for &sum in part.iter() {
            xor ^= sum;
        }
        set.insert(xor);
        return;
    }
    for i in 0..part.len() {
        part[i] += a[index];
        dfs(a, part, index + 1, set);
        part[i] -= a[index];
    }
    part.push(a[index]);
    dfs(a, part, index + 1, set);
    part.pop();
}
