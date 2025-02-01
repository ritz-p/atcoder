use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };
    let mut zeros = vec![0; 3_usize.pow(n as u32 - 1)];
    for (index, c) in s.iter().enumerate() {
        if *c == '0' {
            zeros[index / 3] += 1;
        }
    }
}
