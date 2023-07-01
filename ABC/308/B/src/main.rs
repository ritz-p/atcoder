use proconio::input;
use std::collections::HashMap;
fn main(){
    input!{
        n: usize,
        m: usize,
        c: [String;n],
        d: [String;m],
        p: [usize;m+1]
    };
    let mut res = 0;
    let mut hash:HashMap<String,usize> = HashMap::new();
    for i in 0..m{
        hash.insert(d[i].clone(),p[i+1]);
    }
    for i in 0..n{
        if hash.contains_key(&c[i]){
            res += hash.get(&c[i]).unwrap();
        }else{
            res += p[0];
        }
    }
    println!("{}",res);
}
