use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        ss: [Chars;n]
    };
    let mut v = vec![];
    for s in ss {
        v.push((s.len(), s));
    }
    v.sort_by(|a, b| a.0.cmp(&b.0));
    for s in v {
        print!("{}", s.1.iter().collect::<String>());
    }
    println!();
}
