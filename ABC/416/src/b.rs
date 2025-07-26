use proconio::{input, marker::Chars};
fn main() {
    input! {
        mut s:Chars
    };
    let mut f = true;
    let mut t = s.clone();

    for (index, &c) in s.iter().enumerate() {
        if c == '.' {
            if f {
                t[index] = 'o';
                f = false;
            }
        } else {
            f = true;
        }
    }

    println!("{}", t.into_iter().collect::<String>());
}
