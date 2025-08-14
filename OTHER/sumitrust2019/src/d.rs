use std::collections::HashSet;

use proconio::{input, marker::Chars};
fn main() {
    input! {
        n: usize,
        s: Chars
    };
    let mut fset = HashSet::new();
    let mut sset = vec![HashSet::new(); 10];
    let mut tset = vec![vec![HashSet::new(); 10]; 10];
    for i in 0..n - 2 {
        if !fset.contains(&s[i]) {
            fset.insert(s[i]);
            let si = s[i].to_digit(10).unwrap() as usize;
            for j in i + 1..n - 1 {
                if sset[si].len() == 10 {
                    break;
                }
                if !sset[si].contains(&s[j]) {
                    sset[si].insert(&s[j]);
                    let sj = s[j].to_digit(10).unwrap() as usize;
                    for k in j + 1..n {
                        if tset[si][sj].len() == 10 {
                            break;
                        }
                        if !tset[si][sj].contains(&s[k]) {
                            tset[si][sj].insert(s[k]);
                        }
                    }
                }
            }
        }
    }
    let mut res = 0;
    for i in 0..10 {
        for j in 0..10 {
            res += tset[i][j].len();
        }
    }

    println!("{}", res);
}
