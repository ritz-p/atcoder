use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: String
    };

    let alphabet = ('a'..='z').collect::<Vec<char>>();

    for i in 0..26 {
        let cs = s
            .iter()
            .map(|c| {
                let current = *c as u32 - 48 + i;
                let n = alphabet[current as usize % 26];
                n
            })
            .collect::<String>();
        if cs == t {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
