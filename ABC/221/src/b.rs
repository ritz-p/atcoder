use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        mut t: Chars
    };
    for i in 0..s.len() - 1 {
        if s[i] != t[i] {
            let tmp = t[i];
            t[i] = t[i + 1];
            t[i + 1] = tmp;
            break;
        }
    }
    if s.iter().collect::<String>() == t.iter().collect::<String>() {
        println!("Yes");
    } else {
        println!("No");
    }
}
