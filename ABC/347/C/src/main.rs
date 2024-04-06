use itertools::Itertools;
use proconio::input;

fn main(){
    input!{
        n: usize,
        a: usize,
        b: usize,
        mut d: [usize;n]
    };
    d = d.iter().map(|e| e % (a+b)).sorted().dedup().collect();
    d.push(d[0] + a + b);
    if d.into_iter().tuple_windows().any(|(e,f)| f - e > b){
        println!("Yes");
    }else{
        println!("No");
    }
}
