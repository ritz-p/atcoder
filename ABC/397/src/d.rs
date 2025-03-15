use std::collections::HashMap;

use proconio::input;
fn main() {
    input! {
        n: u128
    };
    let mut set = HashMap::new();
    for i in 1..=1000000 {
        set.insert(i * i * i + n, i);
    }
    for x in 1..=1000000 {
        if let Some(y) = set.get(&(x * x * x)) {
            println!("{} {}", x, y);
            return;
        }
    }
    println!("-1");
}
