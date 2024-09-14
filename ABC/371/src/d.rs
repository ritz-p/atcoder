use proconio::input;

fn main() {
    input! {
        n: usize,
        x: [isize;n],
        p: [usize;n],
        q: usize,
        lr: [(isize,isize);q]
    };

    let mut s = vec![0; n + 1];
    for i in 0..n {
        s[i + 1] = s[i] + p[i];
    }

    for (l, r) in lr {
        let lp = x.partition_point(|&xi| xi < l);
        let rp = x.partition_point(|&xi| xi <= r);

        println!("{}", s[rp] - s[lp]);
    }
}
