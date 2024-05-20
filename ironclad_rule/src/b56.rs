pub const MOD: isize = 1e9 as isize + 7;

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize, q:usize,
        s: Chars,
        lr: [(usize, usize); q],
    };
    let s = s
        .iter()
        .map(|&x| x as isize - 'a' as isize + 1)
        .collect_vec();
    let mut pow = vec![1_isize; n + 1];
    for i in 1..=n {
        pow[i] = pow[i - 1] * 100 % MOD;
    }

    let mut th = vec![0_isize; n + 1];
    let mut rth = vec![0_isize; n + 1];
    for i in 0..n {
        th[i + 1] = (th[i] * 100 + s[i]) % MOD;
        rth[i + 1] = (rth[i] * 100 + s[n - i - 1]) % MOD;
    }

    for (l, r) in lr {
        let t = th[r] - ((th[l - 1] * pow[r - l + 1]) % MOD);
        let t = if t < 0 { t + MOD } else { t };
        let rt = rth[n - l + 1] - ((rth[n - r] * pow[r - l + 1]) % MOD);
        let rt = if rt < 0 { rt + MOD } else { rt };

        if t == rt {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
