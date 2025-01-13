use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        _w:usize
    };
    let mut res = vec![];
    for i in 0..3 {
        for j in 1..100 {
            res.push(j * 100usize.pow(i));
        }
    }
    println!("{}", res.len());
    println!("{}", res.iter().join(" "));
}
