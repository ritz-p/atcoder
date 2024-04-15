use proconio::input;
use itertools::Itertools;
fn main(){
    input!{
        n: usize,
        a: [usize;n]
    }
    for e in a.iter().combinations(3){
        if e[0] + e[1] + e[2] == 1000{
            println!("Yes");
            return;
        }
    }
    println!("No");
}