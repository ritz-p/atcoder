use proconio::input;
use proconio::marker::Chars;
fn main(){
    input!{
        s: Chars,
        mut t: Chars,
    };
    t = t.iter().map(|&c|c.to_lowercase().next().unwrap_or(c)).collect();
    let mut three = false;
    for i in 0..s.len() {
        if s[i] == t[0] {
            for j in i + 1..s.len() {
                if s[j] == t[1] {
                    for k in j + 1..s.len() {
                        if s[k] == t[2] {
                            three = true;
                        }
                    }
                    break;
                }
            }
            break;
        }
    }
    let mut two = false;
    if t[2] == 'x' {
        for i in 0..s.len() {
            if s[i] == t[0] {
                for j in i + 1..s.len() {
                    if s[j] == t[1] {
                        two = true;
                    }
                }
                break;
            }
        }
    }

    if three || two {
        println!("Yes");
    } else {
        println!("No");
    }
}
