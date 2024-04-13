use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;
fn main(){
    input!{
        s: Chars,
    };
    let mut map:HashMap<char, usize> = HashMap::new();
    for c in s{
        *map.entry(c).or_insert(1) += 1;
    }
    let mut v = vec![0;102];

    for m in &map{
        v[*m.1] += 1;
    }
    if v.iter().any(|e| *e != 0 && *e != 2){
        println!("No");
    }else{
        println!("Yes");
    }
}
