use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars
    };
    let mut positions = vec![];
    for (i, &c) in s.iter().enumerate() {
        if c == '1' {
            positions.push(i as i64);
        }
    }
    let l = positions.len();
    let mut d = vec![];

    for (i, &p) in positions.iter().enumerate() {
        d.push(p - i as i64);
    }

    let mid = d[l / 2];
    let mut res = 0;
    for &x in &d {
        res += (x - mid).abs();
    }
    println!("{}", res);
}
