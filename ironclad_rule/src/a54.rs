use proconio::input;
use std::collections::HashMap;
fn main(){
    input!{
        n: usize,
    };
    let mut map = HashMap::new();
    for _i in 0..n{
        input!{
            q: usize,
        };
        match q{
            1 => {
                input!{
                    s: String,
                    m: usize,
                };
                map.insert(s,m);
            },
            2 => {
                input!{
                    s: String,
                };
                if let Some(v) = map.get(&s){
                    println!("{}",v);
                }
            },
            _ => {}
        }
    }
}