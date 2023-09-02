use proconio::input;
use std::collections::HashSet;
fn main(){
    input!{
        n: usize,
    };
    let mut tiles = HashSet::new();
    for i in 0..n{
        input!{
            a: usize,
            b: usize,
            c: usize,
            d: usize,
        }
        for j in a..b{
            for k in c..d{
                let mut x = j.to_string();
                if j < 10{
                    x = "0".to_owned() + &x;
                }
                let mut y = k.to_string();
                if k < 10{
                    y = "0".to_owned() + &y;
                }
                let z = x+&y;
                tiles.insert(z);
            }
        }
    }
    println!("{}",tiles.len());
}
