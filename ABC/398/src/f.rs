use proconio::{input, marker::Chars};
fn main() {
    input! {
        s: Chars,
    };
    let s_str: String = s.iter().collect();
    let l = s_str.len();
    let rev: String = s_str.chars().rev().collect();
    let t = format!("{}#{}", s_str, rev);
    let t_bytes = t.as_bytes();
    let m = t.len();

    let mut pi = vec![0; m];
    for i in 1..m {
        let mut j = pi[i - 1];
        while j > 0 && t_bytes[i] != t_bytes[j] {
            j = pi[j - 1];
        }
        if t_bytes[i] == t_bytes[j] {
            j += 1;
        }
        pi[i] = j;
    }

    let k = pi[m - 1];
    let append: String = s_str[..(l - k)].chars().rev().collect();
    println!("{}{}", s_str, append);
}
