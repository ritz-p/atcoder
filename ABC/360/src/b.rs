use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        s: Chars,
        t: Chars,
    };
    for w in 1..s.len() {
        for c in 0..w {
            let mut r: Vec<char> = vec![];
            let mut count = 0;
            while w * count + c < s.len() {
                r.push(s[w * count + c]);
                count += 1;
            }
            if r.iter().collect::<String>() == t.iter().collect::<String>() {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
