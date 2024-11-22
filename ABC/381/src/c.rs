use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars
    };
    let mut pos = vec![];
    for (index, c) in s.iter().enumerate() {
        if *c == '/' {
            pos.push(index);
        }
    }
    let mut res = 1;
    for e in pos {
        let mut current = 1;
        while e >= current && e + current < n {
            if s[e - current] == '1' && s[e + current] == '2' {
                current += 1;
            } else {
                break;
            }
        }
        res = res.max(1 + (current - 1) * 2);
    }

    println!("{}", res);
}
