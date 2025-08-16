use proconio::{input, marker::Chars};
fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
        t: Chars,
        lr: [(usize,usize);m]
    };
    let mut sums = vec![0; n + 1];
    for (l, r) in lr {
        sums[l - 1] ^= 1;
        sums[r] ^= 1;
    }
    let mut current = 0;
    for i in 0..n {
        current ^= sums[i];
        if current == 0 {
            print!("{}", s[i]);
        } else {
            print!("{}", t[i]);
        }
    }

    println!();
}
