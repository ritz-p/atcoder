use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        st: [(String,usize);n]
    };
    let mut set = HashSet::new();
    let mut m = 0;
    let mut res = 0;
    for (index, (s, t)) in st.iter().enumerate() {
        if !set.contains(s) {
            set.insert(s);
        } else {
            continue;
        }
        if *t > m {
            m = *t;
            res = index;
        }
    }

    println!("{}", res + 1);
}
