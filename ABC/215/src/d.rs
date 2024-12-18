use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize;n]
    };
    let mut flag = vec![true; 100_001];
    flag[0] = false;
    for i in a {
        flag[i] = false;
    }

    let mut v = vec![true; m + 1];
    let mut res = vec![1];
    for i in 2..=m {
        if !v[i] {
            continue;
        }
        if (i..flag.len()).step_by(i).all(|j| flag[j]) {
            res.push(i);
        } else {
            (i..=m).step_by(i).for_each(|j| v[j] = false);
        }
    }

    println!("{}", res.len());
    println!("{}", res.iter().join("\n"));
}
