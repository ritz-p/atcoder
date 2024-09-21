use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        mut m: usize,
    };
    let mut current = 20;
    let mut v = vec![];
    while m > 0 {
        if 3_usize.pow(current) > m {
            current -= 1;
        } else {
            m -= 3_usize.pow(current);
            v.push(current);
        }
    }

    println!("{}", v.len());
    println!("{}", v.iter().join(" "));
}
