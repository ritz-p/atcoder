use std::isize;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [isize;n]
    };
    let mut minus = 0;
    let mut zero = 0;
    let mut min = isize::MAX;

    for e in &a {
        if e.is_negative() {
            minus += 1;
        } else if *e == 0 {
            zero += 1;
        }
        min = min.min(e.abs());
    }
    let res = a.iter().map(|e| e.abs()).sum::<isize>();

    if minus % 2 == 0 || zero < 0 {
        println!("{}", res);
    } else {
        println!("{}", res - min * 2);
    }
}
