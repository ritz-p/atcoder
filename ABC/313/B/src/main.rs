use proconio::input;
use itertools::Itertools;
fn main() {
    input! {n: usize, ab: [(usize, usize)]}
    let mut ans = vec![];
    for i in 0..n {
        let n = ab.iter().filter(|(_, node)| *node == i+1).collect_vec();
        if n.len() == 0 {
            ans.push(i + 1);
        }
    }
    println!("{}", if ans.len() == 1 {ans[0].to_string()} else {"-1".to_string()});
}
