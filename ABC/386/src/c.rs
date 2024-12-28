use proconio::{input, marker::Chars};

fn main() {
    input! {
        _k: usize,
        s: Chars,
        mut t: Chars,
    };
    let sl = s.len();
    let tl = t.len();
    if sl == tl {
        for i in 0..sl {
            if s[i] != t[i] {
                t[i] = s[i];
                break;
            }
        }
        if s.iter().collect::<String>() == t.iter().collect::<String>() {
            println!("Yes");
        } else {
            println!("No");
        }
    } else if sl - 1 == tl {
        for i in 0..sl - 1 {
            if s[i] != t[i] {
                t.insert(i, s[i]);
                break;
            }
            if i == sl - 2 && t.len() != sl {
                t.push(s[i + 1]);
            }
        }
        if s.iter().collect::<String>() == t.iter().collect::<String>() {
            println!("Yes");
        } else {
            println!("No");
        }
    } else if sl + 1 == tl {
        for i in 0..sl {
            if s[i] != t[i] {
                t.remove(i);
                break;
            }
            if i == sl - 1 && t.len() != sl {
                t.remove(i + 1);
            }
        }
        if s.iter().collect::<String>() == t.iter().collect::<String>() {
            println!("Yes");
        } else {
            println!("No");
        }
    } else {
        println!("No");
    }
}
