use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let l = s.len();
    let mut res = 0;
    for i in 0..l {
        if s[i] == 'A' {
            for j in 1..=100 {
                if i + j * 2 < l {
                    if s[i + j] == 'B' && s[i + j * 2] == 'C' {
                        res += 1;
                    }
                }
            }
        }
    }

    println!("{}", res);
}
