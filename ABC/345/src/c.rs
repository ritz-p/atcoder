use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;
fn main(){
    input!{
        s: Chars,
    };
    let mut map:HashMap<char,usize> = HashMap::new();
    for c in &s{
        *map.entry(*c).or_insert(0) += 1;
    }
    let mut res = s.len() * (s.len() - 1) / 2;

    let mut flag = true;
    for (_key,value) in map{
        if value > 1{
            if flag{
                flag = false;
                res -= value * (value - 1) / 2 - 1;
            }else{
                res -= value * (value - 1) / 2;
            }
        }
    }
    println!("{}",res);
}
