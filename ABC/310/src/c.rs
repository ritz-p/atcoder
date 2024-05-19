use proconio::input;
use std::collections::HashSet;
use std::iter::FromIterator;

fn main(){
    input!{
        n: usize,
        s: [String;n]
    };
    let mut hash:HashSet<String> = HashSet::from_iter(s.iter().cloned());
    let mut rm:HashSet<String> = HashSet::new();
    for b in &hash{
        if b.chars().count() == 1{
            continue
        }
        if rm.contains(b){
            continue
        }
        let reversed = b.chars().rev().collect::<String>();
        if b == &reversed{
            continue
        }
        if hash.contains(&reversed){
            rm.insert(reversed.clone());
        }
    }

    for e in &rm{
        hash.remove(e);
    }
    println!("{}",hash.len());
}
