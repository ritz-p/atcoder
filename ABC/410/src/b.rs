use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        n: usize,
        q: usize,
        x: [usize;q]
    };
    let mut v = vec![0; n];
    let mut res = vec![];
    for e in x {
        match e {
            0 => {
                let mut current = 0;
                for i in 0..n {
                    if v[i] < v[current] {
                        current = i;
                    }
                }
                v[current] += 1;
                res.push(current + 1);
            }
            n => {
                v[n - 1] += 1;
                res.push(n);
            }
        }
    }

    println!("{}", res.iter().join(" "));
}
