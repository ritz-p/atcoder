use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars
    };
    let n = s.len();
    let mut index = 0;
    let mut current = 0;
    loop {
        if s[index] == 'W' {
            current += 1;
        } else if s[index] == 'A' && current > 0 {
            s[index - current] = 'A';
            for i in 0..current {
                s[index - i] = 'C';
            }
            current = 0;
        } else {
            current = 0;
        }
        index += 1;
        if index >= n {
            break;
        }
    }
    println!("{}", s.iter().collect::<String>());
}
