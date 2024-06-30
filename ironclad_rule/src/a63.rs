use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize,usize);m]
    };
    let mut v = vec![vec![]; n];
    for (a, b) in ab {
        v[a - 1].push(b - 1);
        v[b - 1].push(a - 1);
    }
    let mut counts = vec![-1; n];
    counts[0] = 0;
    let mut visited = vec![false; n];
    visited[0] = true;
    let mut count = 0;
    let mut current = v[0].clone();
    loop {
        count += 1;
        let mut next = vec![];
        for i in current {
            if !visited[i] {
                for k in &v[i] {
                    next.push(*k);
                }
                visited[i] = true;
                counts[i] = count;
            }
        }
        if next.is_empty() {
            break;
        }
        current = next;
    }
    println!("{}", counts.iter().join("\n"));
}
