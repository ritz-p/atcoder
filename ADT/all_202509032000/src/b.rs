use proconio::{input, marker::Chars};
fn main() {
    input! {
        _n: usize,
        s: Chars
    };
    let mut a = false;
    let mut b = false;
    let mut c = false;
    for (index, e) in s.iter().enumerate() {
        match e {
            'A' => {
                a = true;
            }
            'B' => {
                b = true;
            }
            'C' => {
                c = true;
            }
            _ => {}
        }
        if a && b && c {
            println!("{}", index + 1);
            return;
        }
    }
}
