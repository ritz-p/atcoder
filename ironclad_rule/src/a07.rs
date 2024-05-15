use proconio::input;

fn main(){
    input! {
        d: usize,
        n: usize,
        lr: [(usize,usize);n]
    };
    let mut at = vec![0;d+2];
    for i in 0..n{
        at[lr[i].0] += 1;
        at[lr[i].1 + 1] -= 1;
    }

    let mut res = vec![0;d+1];
    for i in 1..=d{
        res[i] = res[i-1] + at[i];
    }
    for i in 1..=d{
        println!("{}",res[i]);
    }
}