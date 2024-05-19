use proconio::input;
use itertools::Itertools;
fn main(){
    input!{
        n: usize,
    };
    let mut repunit = vec![];
    let mut current = "".to_string();
    for i in 0..13{
        current += "1";
        let num:usize = current.parse().unwrap();
        repunit.push(num);
        repunit.push(num);
        repunit.push(num);
    }
    let mut combi = vec![];
    for perm in repunit.iter().permutations(3){
        combi.push(perm[0]+perm[1]+perm[2]);
    }
    combi.sort();
    combi.dedup();
    println!("{}",combi[n-1]);
}