use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        _n: isize,
        a: isize,
        b: isize,
        p: isize,
        q: isize,
        r: isize,
        s: isize,
    };
    let mut res = vec![vec!['.'; (s - r + 1) as usize]; (q - p + 1) as usize];

    let kl = (p - a).max(r - b);
    let kr = (q - a).min(s - b);

    for i in kl..=kr {
        res[(a + i - p) as usize][(b + i - r) as usize] = '#';
    }

    let ll = (p - a).max(b - s);
    let lr = (q - a).min(b - r);
    for i in ll..=lr {
        res[(a + i - p) as usize][(b - i - r) as usize] = '#';
    }

    for c in res {
        println!("{}", c.iter().join(""));
    }
}
