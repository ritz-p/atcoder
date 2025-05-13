use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Chars;2 * n]
    };
    let mut res = vec![];
    for i in 0..n * 2 {
        res.push((i, 0));
    }
    for i in 0..m {
        for j in 0..n {
            let (one, two) = (a[res[2 * j].0][i], a[res[2 * j + 1].0][i]);
            if one == two {
                continue;
            }
            match (one, two) {
                ('G', 'C') | ('P', 'G') | ('C', 'P') => {
                    res[2 * j].1 += 1;
                }
                _ => {
                    res[2 * j + 1].1 += 1;
                }
            }
            res.sort_by(|a, b| {
                let ord = b.1.cmp(&a.1);
                if ord == std::cmp::Ordering::Equal {
                    a.0.cmp(&b.0)
                } else {
                    ord
                }
            });
        }
    }
    println!("{}", res.iter().map(|v| v.0 + 1).join("\n"));
}
