use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut n: Chars
    };

    n.sort();

    let mut res = 0;
    for perm in n.iter().permutations(n.len()) {
        if perm[0] == &'0' {
            continue;
        }
        for i in 1..n.len() {
            let l = &perm[0..i];
            let r = &perm[i..n.len()];
            if l[0] == &'0' || r[0] == &'0' {
                continue;
            }
            let nl: usize = l.iter().map(|c| *c).collect::<String>().parse().unwrap();
            let nr: usize = r.iter().map(|c| *c).collect::<String>().parse().unwrap();
            res = res.max(nl * nr);
        }
    }

    println!("{}", res);
}
