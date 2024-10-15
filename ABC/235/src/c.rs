use proconio::input;
use std::collections::HashMap;
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize;n],
        xk: [(usize,usize);q]
    };
    let mut map: HashMap<usize, Vec<usize>> = HashMap::new();

    for (index, e) in a.iter().enumerate() {
        if let Some(v) = map.get_mut(e) {
            v.push(index);
        } else {
            map.insert(*e, vec![index]);
        }
    }
    for (x, k) in xk {
        if let Some(v) = map.get(&x) {
            if v.len() > k - 1 {
                println!("{}", v[k - 1] + 1);
            } else {
                println!("-1");
            }
        } else {
            println!("-1");
        }
    }
}
