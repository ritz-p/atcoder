use proconio::input;
use std::collections::VecDeque;
fn main(){
    input!{
        n: usize,
        m: usize,
        mut a: [usize;m]
    };
    let mut fire:VecDeque<usize> = VecDeque::new();
    for i in &a{
        fire.push_back(*i);
    }
    for i in 1..n{
        if i == fire[0]{
            fire.pop_front();
            println!("0");
        }else if i < fire[0]{
            println!("{}",fire[0]-i);
        }
    }
    println!("0");
}
