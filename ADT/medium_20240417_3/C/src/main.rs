use proconio::input;
use std::collections::HashSet;
use std::iter::FromIterator;
fn main(){
    input!{
        n: usize,
        a: [usize;n]
    };
    let set:HashSet<usize> = HashSet::from_iter(a.iter().cloned());
    for i in 0..=2000{
        if !set.contains(&i){
            println!("{}",i);
            return;
        }
    }
}
