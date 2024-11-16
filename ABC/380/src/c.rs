use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        k: usize,
        mut s: Chars,
    };

    let mut v = vec![];
    let mut current = 0;
    let mut pos = 0;
    for (index, c) in s.iter().enumerate() {
        match c {
            '0' => {
                if current > 0 {
                    v.push((pos, current));
                    current = 0;
                }
            }
            '1' => {
                if current == 0 {
                    pos = index;
                }
                current += 1;
            }
            _ => {}
        }
        if index == n - 1 && current > 0 {
            v.push((pos, current));
        }
    }

    for i in 0..v[k - 2].0 + v[k - 2].1 {
        print!("{}", s[i]);
    }
    for _i in 0..v[k - 1].1 {
        print!("1");
    }
    for i in v[k - 2].0 + v[k - 2].1..v[k - 1].0 {
        print!("{}", s[i]);
    }
    for i in v[k - 1].0 + v[k - 1].1..n {
        print!("{}", s[i]);
    }
}
