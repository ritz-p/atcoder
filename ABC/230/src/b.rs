use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    };
    let base = vec!['o', 'x', 'x'];

    for i in 0..3 {
        if s.iter()
            .enumerate()
            .all(|(index, c)| *c == base[(index + i) % 3])
        {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
