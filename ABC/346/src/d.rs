use proconio::{input, marker::Chars};
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        s: Chars,
        c: [usize; n],
    }
    let mut a = Vec::<usize>::new();
    let mut b = Vec::<usize>::new();
    let mut sum_a = 0;
    let mut sum_b = 0;
    let mut t = '1';
    for i in 0..n {
        if s[i] == t {
            sum_b += c[i];
            b.push(sum_b);
            a.push(sum_a);
        } else {
            sum_a += c[i];
            a.push(sum_a);
            b.push(sum_b);
        }
        t = if t == '0' { '1' } else { '0' };
    }
    let mut ans = sum_a+sum_b;
    for i in 0..(n-1) {
        ans = min(ans, a[i]+b[b.len()-1]-b[i]);
        ans = min(ans, b[i]+a[a.len()-1]-a[i]);
    }
    println!("{}", ans);
}