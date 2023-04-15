use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut q2 = vec![vec![]; n + 1];
    let mut q3 = vec![vec![]; 200_001];
    for _ in 0..q {
        input! {
            op: u8,
            i: usize,
        }
        if op == 1 {
            input! {
                j: usize,
            }
            q2[j].push(i);
            q3[i].push(j);
        } else if op == 2 {
            q2[i].sort();
            println!("{}", q2[i].iter().join(" "));
        } else {
            q3[i].sort();
            q3[i].dedup();
            println!("{}", q3[i].iter().join(" "));
        }
    }
}
