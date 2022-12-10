use proconio::input;
use itertools::Itertools;

fn main(){
    input!{
        n:usize,
        k:usize,
        d:usize,
        mut arr:[usize;n],
    };
    let mut results = vec![];
    for comb in (0..n).combinations(k){
        // println!("{:?}",comb);
        let mut total = 0;
        for i in 0..k{
            total += arr[comb[i]]
        }
        results.push(total);
    }
    let mut max:isize = -1;
    for i in 0..results.len(){
        if results[i] % d == 0 && results[i] as isize > max{
            max = results[i] as isize;
        }
    }
    println!("{}",max);
}