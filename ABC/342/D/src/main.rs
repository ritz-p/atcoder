use proconio::input;
use std::collections::HashSet;
fn main(){
    input!{
        n: usize,
        a: [usize;n]
    };
    let mut hash = HashSet::new();
    for i in 0..100000{
        hash.insert(i*i);
    }
    let mut res = 0;
    for i in 0..n-1{
        for j in i..n{
            if i == j{
                continue;
            }
            if i == 0 || j == 0{
                res += 1;
                continue;
            }
            let calc = a[i] * a[j];
            if hash.contains(&calc){
                res += 1;
            }
        }
    }
    println!("{}",res);
}
