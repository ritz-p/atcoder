use itertools::Itertools;
use proconio::input;
use superslice::Ext;
fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize;n],
        xy: [(usize,usize);q]
    };
    a.sort();
    let mut res = vec![];

    for (x, y) in xy {
        let mut left = x;
        let mut right = x + y + n;

        while left < right {
            let mid = (left + right) / 2;
            let l = a.lower_bound(&x);
            let r = a.lower_bound(&mid.saturating_add(1));
            let pre = r - l;
            let total = mid - x + 1;
            if total - pre >= y {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        res.push(left);
    }

    println!("{}", res.iter().join("\n"));
}
