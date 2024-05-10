use proconio::input;
use std::collections::HashMap;
fn main(){
    input!{
        n: usize,
        m: usize,
    };
    let mut graph:Vec<HashMap<usize,usize>> = vec![HashMap::new();n];
    for i in 0..m{
        input!{
            k: usize,
            c: usize,
            a: [usize;k]
        };
        
    }
}
