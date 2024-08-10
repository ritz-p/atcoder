use proconio::input;
use std::collections::HashMap;
fn main() {
    input! {
        q: usize,
    };

    let mut map = HashMap::new();
    for _i in 0..q {
        input! {
            n: usize,
        };
        match n {
            1 => {
                input! {
                    x: usize,
                };
                *map.entry(x).or_insert(0) += 1;
            }
            2 => {
                input! {
                    x: usize,
                };
                if let Some(v) = map.get_mut(&x) {
                    if *v == 1 {
                        map.remove(&x);
                    } else {
                        *v -= 1;
                    }
                }
            }
            3 => {
                println!("{}", map.len());
            }
            _ => {}
        }
    }
}
