use proconio::input;
use std::collections::HashSet;
fn main(){
    input!{
        n: usize,
        x: isize,
        arr: [isize;n]
    };
    let mut hash = HashSet::new();
    for i in 0..n{
        hash.insert(arr[i]);
    }
    for i in 0..n{
        if hash.contains(&(arr[i] - x)){
            println!("Yes");
            return
        }
    }
    println!("No");
}
