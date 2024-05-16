use proconio::input;

fn main(){
    input! {
        t: usize,
        n: usize,
        lr: [(usize,usize);n]
    };
    let mut at = vec![0;t+1];
    for (l,r) in lr{
        at[l] += 1;
        at[r] -= 1;
    }

    let mut res = vec![0;t+1];
    res[0] = at[0];
    for i in 1..t{
        res[i] += res[i-1] + at[i];
    }

    for i in 0..t{
        println!("{}",res[i]);
    }
}