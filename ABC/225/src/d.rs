use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    };
    let mut v = vec![(usize::MAX, usize::MAX); n];

    for _i in 0..q {
        input! {
            t: usize,
        };
        match t {
            1 => {
                input! {
                    x: usize,
                    y: usize,
                };
                v[x - 1].1 = y - 1;
                v[y - 1].0 = x - 1;
            }
            2 => {
                input! {
                    x: usize,
                    y: usize,
                };
                v[x - 1].1 = usize::MAX;
                v[y - 1].0 = usize::MAX;
            }
            3 => {
                input! {
                    x: usize
                };
                let mut f = x - 1;
                while v[f].0 != usize::MAX {
                    f = v[f].0;
                }
                let mut res = vec![];
                res.push(f);
                while v[f].1 != usize::MAX {
                    res.push(v[f].1);
                    f = v[f].1;
                }
                println!("{} {}", res.len(), res.iter().map(|v| v + 1).join(" "));
            }
            _ => {}
        }
    }
}
