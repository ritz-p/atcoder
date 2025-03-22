use std::collections::HashMap;

use proconio::input;
fn main() {
    input! {
        a: [usize;7]
    };
    let mut map = HashMap::new();

    for e in &a {
        *map.entry(e).or_insert(0) += 1;
    }
    let mut vs = vec![0; 8];
    for (_k, v) in map {
        vs[v] += 1;
    }
    if vs[3] > 1 {
        println!("Yes");
    } else if vs[2] > 0 && vs[3] > 0 {
        println!("Yes");
    } else if vs[2] > 0 && vs[4] > 0 {
        println!("Yes");
    } else if vs[2] > 0 && vs[5] > 0 {
        println!("Yes");
    } else if vs[3] > 0 && vs[4] > 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
