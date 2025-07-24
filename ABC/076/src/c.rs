use proconio::{input, marker::Chars};
fn main() {
    input! {
        s: Chars,
        t: Chars,
    };
    let sl = s.len();
    let tl = t.len();
    if sl < tl {
        println!("UNRESTORABLE");
        return;
    }

    for i in (0..=sl - tl).rev() {
        let mut f = true;
        for j in 0..tl {
            if s[i + j] != '?' && s[i + j] != t[j] {
                f = false;
                break;
            }
        }
        if f {
            let mut v = s.clone();
            for k in i..i + tl {
                v[k] = t[k - i];
            }
            let res = v
                .into_iter()
                .map(|c| if c == '?' { 'a' } else { c })
                .collect::<String>();
            println!("{}", res);
            return;
        }
    }
    println!("UNRESTORABLE");
}
