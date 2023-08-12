use proconio::input;
use std::collections::HashSet;
use std::iter::FromIterator;
fn main(){
    input!{
        n: usize,
    };
    let mut p = vec![];
    let mut numbers:Vec<HashSet<usize>> = vec![];
    for _ in 0..n{
        input!{
            m: usize,
            points: [usize;m],
        };
        p.push(m);
        let hash = HashSet::from_iter(points.iter().cloned());
        numbers.push(hash);
    }
    let mut res = vec![];
    let mut min = 101;
    input!{
        x: usize,
    }
    for i in 0..n{
        if numbers[i].contains(&x){
            if p[i] < min{
                res = vec![];
                min = p[i];
                res.push(i+1);
            }else if p[i] == min{
                res.push(i+1);
            }
        }
    }

    println!("{}",res.len());
    for i in 0..res.len(){
        print!("{} ",res[i]);
    }
    println!();
}
