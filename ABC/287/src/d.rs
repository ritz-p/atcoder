use proconio::{input, marker::Chars};
fn main() {
    input! {
        s: Chars,
        t: Chars
    };
    let sl = s.len();
    let tl = t.len();

    let mut pre = vec![false; sl + 1];
    pre[0] = true;
    for i in 0..tl {
        if !check(s[i], t[i]) {
            break;
        }
        pre[i + 1] = true;
    }

    let sr = s.iter().rev().cloned().collect::<Vec<char>>();
    let tr = t.iter().rev().cloned().collect::<Vec<char>>();

    let mut suf = vec![false; sl + 1];
    suf[0] = true;
    for i in 0..tl {
        if !check(sr[i], tr[i]) {
            break;
        }
        suf[i + 1] = true;
    }

    for x in 0..=tl {
        if pre[x] && suf[tl - x] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

fn check(a: char, b: char) -> bool {
    a == b || a == '?' || b == '?'
}
