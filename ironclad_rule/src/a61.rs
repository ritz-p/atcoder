use itertools::Itertools;
use proconio::input;

fn main(){
    input!{
        n: usize,
        m: usize,
        ab: [(usize,usize);m]
    };
    let mut atob = vec![vec![];n];

    for (a,b) in ab{
        atob[a-1].push(b);
        atob[b-1].push(a);
    }

    for i in 0..n{
        println!("{}: {{{}}}",i+1,atob[i].iter().join(", ").to_string());
    }
}