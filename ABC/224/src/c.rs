use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(isize,isize);n]
    };
    let mut res = 0;
    for comb in (0..n).combinations(3) {
        let (one, two, three) = (comb[0], comb[1], comb[2]);
        if (xy[three].0 - xy[one].0) * (xy[two].1 - xy[one].1)
            - (xy[two].0 - xy[one].0) * (xy[three].1 - xy[one].1)
            != 0
        {
            res += 1;
        }
    }
    println!("{}", res);
}
