use proconio::input;
use std::iter::Iterator;
fn main(){
    input!{
        m: usize,
        d: [usize;m]
    };
    let sum: usize = d.iter().sum();
    let mut half = sum / 2 + 1;
    for i in 0..m{
        if d[i] >= half{
            println!("{} {}",i+1,half);
            return;
        }else{
            half -= d[i];
        }
    }
}
