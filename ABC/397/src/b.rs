use proconio::{input, marker::Chars};
fn main() {
    input! {
        s: Chars,
    };
    let mut current = 0;

    for (index, c) in s.iter().enumerate() {
        if (index + 1 + current) % 2 == 0 && *c == 'i' {
            current += 1;
        } else if (index + 1 + current) % 2 == 1 && *c == 'o' {
            current += 1;
        }
    }
    if (current + s.len()) % 2 == 1 {
        current += 1;
    }

    println!("{}", current);
}
