use proconio::input;

fn main()
{
    input!
    {
        n: usize,
        w: usize,
        mut a: [(usize, usize); n]
    }
    a.sort();
    let mut dp = vec![0; w + 1];
    for (ww, vv) in a
    {
        for i in (ww ..= w).rev()
        {
            dp[i] = dp[i].max(dp[i - ww] + vv as u64);
            println!("{} , {}",dp[i-ww],dp[i]);
        }
    }
    println!("{}", dp[w]);
}
