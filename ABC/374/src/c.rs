use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: [usize;n]
    };
    let mut res = usize::MAX;
    let sum = k.clone().iter().sum::<usize>();
    for i in 0..n {
        for comb in k.iter().combinations(i) {
            let current = comb.into_iter().sum::<usize>();
            res = res.min(current.max(sum - current));
        }
    }
    println!("{}", res);
}
