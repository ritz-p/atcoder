use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        qs: [(usize,usize);q]
    };
    let mut current = 0;
    for (query, index) in &qs {
        match query {
            1 => {
                current += index;
            }
            2 => {
                println!("{}", s[((n + index - 1) - (current % n)) % n]);
            }
            _ => {}
        }
    }
}
