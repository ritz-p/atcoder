use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(isize,isize);n]
    };
    let mut res = 0;
    for comb in (0..n).combinations(3) {
        if (xy[comb[2]].0 - xy[comb[0]].0) * (xy[comb[1]].1 - xy[comb[0]].1)
            - (xy[comb[1]].0 - xy[comb[0]].0) * (xy[comb[2]].1 - xy[comb[0]].1)
            != 0
        {
            res += 1;
        }
    }
    println!("{}", res);
}
