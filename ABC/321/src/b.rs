use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n - 1],
    }
    for i in 0..=100 {
        let mut b = a.clone();
        b.push(i);
        b.sort();
        let s = b[1..n - 1].iter().sum::<usize>();
        if s >= x {
            println!("{}", i);
            return;
        }
    }
    println!("-1");
}
