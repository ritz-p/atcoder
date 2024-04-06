use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;
fn main(){
    input!{
        s: Chars,
    };
    let mut set:HashSet<String> = HashSet::new();

    for i in 0..s.len()+1{
        for j in 0..s.len()+1-i{
            if s[j..j+i].iter().collect::<String>()!= "".to_string(){
                set.insert(s[j..j+i].iter().collect());
            }
        }
    }
    print!("{}",set.len());

}
