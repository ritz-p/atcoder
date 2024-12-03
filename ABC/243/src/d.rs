use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut x: usize,
        s: Chars
    };
    let mut v = vec![];
    for i in 0..n {
        if let Some(c) = v.get(v.len() - 1) {
            if s[i] == 'U' && matches!(c, 'L' | 'R') {
                v.pop().unwrap();
                continue;
            }
        }
        v.push(s[i]);
    }

    for c in v {
        match c {
            'U' => {
                x /= 2;
            }
            'R' => {
                x *= 2;
                x += 1;
            }
            'L' => {
                x *= 2;
            }
            _ => {}
        }
    }
    println!("{}", x);
}
