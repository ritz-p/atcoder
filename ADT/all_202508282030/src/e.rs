use proconio::{input, marker::Chars};
fn main() {
    input! {
        n: usize,
        s: Chars
    };
    let mut count = 0;
    let mut index = 0;
    let mut res = 0;
    while index < n {
        match s[index] {
            '-' => {
                res = res.max(count);
                count = 0;
            }
            'o' => {
                count += 1;
            }
            _ => {}
        }
        index += 1;
    }
    let s = s.iter().rev().collect::<Vec<_>>();
    index = 0;
    count = 0;
    while index < n {
        match s[index] {
            '-' => {
                res = res.max(count);
                count = 0;
            }
            'o' => {
                count += 1;
            }
            _ => {}
        }
        index += 1;
    }
    if res == 0 {
        println!("-1");
    } else {
        println!("{}", res);
    }
}
