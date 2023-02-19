use itertools::Itertools;
use proconio::input;
 
fn main() {
    input! {
        n: usize,
        m: usize,
        re: [usize; m],
    }
 
    let mut ans = vec![];
    let mut j = 0;
    let mut l = 1;
    for i in 1..=n {
        if j < m && re[j] == i {
            j += 1;
        } else {
            ans.extend((l..=i).rev());
            l = i + 1;
        }
    }
    println!("{}", ans.iter().join(" "));
}