use proconio::{input, marker::Chars};
fn main() {
    input! {
        _n: usize,
        s: Chars,
    };
    let mut a = vec![];
    for (index, c) in s.iter().enumerate() {
        if *c == 'A' {
            a.push(index as isize);
        }
    }
    let mut even = 0;

    for (index, p) in a.iter().enumerate() {
        let d = index as isize * 2;
        even += (p - d).abs();
    }
    let mut odd = 0;

    for (index, p) in a.iter().enumerate() {
        let d = index as isize * 2 + 1;
        odd += (p - d).abs();
    }

    println!("{}", even.min(odd));
}
