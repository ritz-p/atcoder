use proconio::{input, marker::Chars};

fn main() {
    input! {
        s1: Chars,
        s2: Chars,
    };
    let mut count = 0;
    if s1[0] == '#' {
        count += 1;
    }
    if s1[1] == '#' {
        count += 1;
    }
    if s2[0] == '#' {
        count += 1;
    }
    if s2[1] == '#' {
        count += 1;
    }
    if count == 2 && (s1[0] == s2[1] || s1[1] == s2[0]) {
        println!("No");
    } else {
        println!("Yes");
    }
}
