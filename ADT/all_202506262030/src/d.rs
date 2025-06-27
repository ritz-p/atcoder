use proconio::{input, marker::Chars};
fn main() {
    input! {
        s: Chars,
        t: Chars,
    };

    let n = s.len();
    if s.iter().collect::<String>() == t.iter().collect::<String>() {
        println!("Yes");
        return;
    }

    for i in 0..n - 1 {
        let mut res = s.clone();
        res.swap(i, i + 1);

        if res.iter().collect::<String>() == t.iter().collect::<String>() {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
