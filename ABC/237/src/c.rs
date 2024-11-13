use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars
    };
    let mut a = 0;
    let mut fa = 0;
    for i in 0..s.len() {
        if s[s.len() - 1 - i] == 'a' {
            a += 1;
        } else {
            break;
        }
    }

    for i in 0..s.len() {
        if s[i] == 'a' {
            fa += 1;
        } else {
            break;
        }
    }

    if fa > a {
        println!("No");
        return;
    }
    let va = vec!['a'; a - fa];
    if !va.is_empty() {
        s.splice(0..0, va);
    }

    if s.iter().collect::<String>() == s.iter().rev().collect::<String>() {
        println!("Yes");
    } else {
        println!("No");
    }
}
