use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    };

    let l = s.len();
    let mut current = 0;
    let mut res = 0;
    while current < l {
        if current < l - 1 && s[current] == '0' && s[current + 1] == '0' {
            current += 2;
            res += 1;
        } else {
            current += 1;
            res += 1;
        }
    }

    println!("{}", res);
}
