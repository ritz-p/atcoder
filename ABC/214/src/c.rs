use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [u64; n],
        t: [u64; n],
    }

    let mut res = vec![10_000_000_000; n];

    let mut best = 10_000_000_000;
    for ii in 0..(2 * n) {
        let i = ii % n;

        best = t[i].min(best);
        res[i] = best.min(res[i]);

        best += s[i];
    }

    for resi in res {
        println!("{}", resi);
    }
}
