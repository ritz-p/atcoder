use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize;n],
        q: [usize;n]
    };
    let mut qmap = HashMap::new();
    let mut imap = HashMap::new();
    for i in 1..=n {
        qmap.insert(q[i - 1], p[i - 1]);
        imap.insert(i, q[i - 1]);
    }

    for i in 1..=n {
        if let Some(pi) = qmap.get(&i) {
            if let Some(qi) = imap.get(&pi) {
                print!("{} ", qi);
            }
        }
    }
    println!();
}
