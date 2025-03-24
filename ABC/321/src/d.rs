use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        p: usize,
        a: [usize;n],
        b: [usize;m]
    };
    let mut bmap: BTreeMap<usize, usize> = BTreeMap::new();

    for i in &a {
        *bmap.entry(*i).or_insert(0) += 1;
    }
    let mut sums = vec![(0_usize, 0_usize, 0_usize)];

    for (i, (k, v)) in bmap.iter().enumerate() {
        sums.push((*k, sums[i].1 + v, sums[i].2 + v * k));
    }

    let mut res = 0;
    let l = sums.len();
    for e in b {
        let mut high = l - 1;
        let mut low = 0;
        let target = p.saturating_sub(e);
        while high > low {
            let mid = (low + high + 1) / 2;
            if sums[mid].0 >= target {
                high = mid - 1;
            } else {
                low = mid;
            }
        }
        res += sums[low].1 * e + sums[low].2;
        res += (n - sums[low].1) * p;
    }

    println!("{}", res);
}
