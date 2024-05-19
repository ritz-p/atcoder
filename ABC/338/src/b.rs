use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;
fn main(){
    input!{
        s: Chars,
    };
    let mut hash:HashMap<char,i32> = HashMap::new();
    for i in 0..s.len(){
        if hash.contains_key(&s[i]){
            *hash.get_mut(&s[i]).unwrap() += 1;
        }else{
            hash.insert(s[i],1);
        }
    }
    let mut v: Vec<(&char, &i32)> = hash.iter().collect();
    v.sort_by(|x,y| {
        if x.1 == y.1{
            x.0.cmp(&y.0)
        }else{
            y.1.cmp(&x.1)
        }
    });
    println!("{}",v[0].0);
}
