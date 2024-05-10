use proconio::input;

fn main(){
    input!{
        n: usize,
        ab: [(usize,usize);n]
    };
    let mut sum = 0;
    for i in 0..n{
        sum += ab[i].0;
    }
    let mut res = sum;
    for i in 0..n{
        res = res.max(sum - ab[i].0 + ab[i].1);
    }

    println!("{}",res);
}
