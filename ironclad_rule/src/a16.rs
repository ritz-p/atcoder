use proconio::input;

fn main(){
    input!{
        n: usize,
        mut a: [usize;n-1],
        mut b: [usize;n-2],
    };
    let mut dp = vec![0;n+1];
    let mut aa = vec![0,0];
    aa.append(&mut a);
    let mut ab = vec![0,0,0];
    ab.append(&mut b);

    dp[1] = 0;
    dp[2] = aa[2];
    for i in 3..=n{
        dp[i] = (dp[i-1]+aa[i]).min(dp[i-2]+ab[i]);
    }
    println!("{}",dp[n]);
}