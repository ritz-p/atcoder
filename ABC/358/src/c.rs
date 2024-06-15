use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;
fn main(){
    input!{
        n: usize,
        m: usize,
        s: [Chars;n]
    };
    let mut g = vec![vec![];n];
    for i in 0..n{
        for j in 0..m{
            if s[i][j] == 'o'{
                g[i].push(j);
            }
        }
    }
    for i in 1..=n{
        for comb in (0..n).combinations(i){
            let mut v = vec![];
            for c in comb{
                v.extend(&g[c]);
            }
            let set:HashSet<&usize> = v.iter().collect();
            if set.len() == m{
                println!("{}",i);
                return;
            }
        }
    }
}
