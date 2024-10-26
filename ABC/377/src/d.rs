use proconio::input;
fn main() {
    input! {
        n: usize,
        m: usize,
        lr: [(usize,usize);n]
    };
    const INF: usize = std::usize::MAX;
    let mut diff = vec![INF; m + 2];
    for &(l, r) in &lr {
        if diff[l] > r {
            diff[l] = r;
        }
    }
    for l in (1..=m).rev() {
        diff[l] = diff[l].min(diff[l + 1]);
    }
    let mut ranges: usize = 0;
    for l in 1..=m {
        if diff[l] != INF {
            let count = (m - diff[l] + 1) as usize;
            ranges += count;
        }
    }
    let total = (m as usize) * (m as usize + 1) / 2;
    let res = total - ranges;
    println!("{}", res);
}
