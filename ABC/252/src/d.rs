use itertools::Itertools;
use proconio::input;
fn main(){
    input!{
        n: usize,
        a: [usize;n]
    };
    let mut res = n * (n - 1) * (n - 2) / 6;

    for (_index,num) in a.iter().counts(){
        if num >= 2{
            res -= num * (num - 1) * (num - 2) / 6;
            res -= (n - num) * num * (num - 1) / 2;
        }
    }

    println!("{}",res);
}
