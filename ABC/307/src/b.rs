use itertools::Itertools;
use proconio::*;
fn main(){
    input!{
        n: usize,
        arr: [String;n]
    };
    for i in 0..n{
        for j in 0..n{
            if i == j{
                continue;
            }
            let mut s:String = arr[i].clone();
            s += &arr[j];
            if s.len() % 2 == 0{
                let a:String = (&s[..s.len()/2]).to_string();
                let b:String = (&s[s.len()/2..]).to_string().chars().rev().collect::<String>();
                if a == b{
                    println!("Yes");
                    return;
                }
            }else{
                let a:String = (&s[..(s.len()-1)/2]).to_string();
                let b:String = (&s[(s.len()-1)/2+1..]).to_string().chars().rev().collect::<String>();
                if a == b{
                    println!("Yes");
                    return; 
                }
            }
        }
    }
    println!("No");
}
