use proconio::input;

fn main() {
    input!{
        n: usize,
        tup: [(usize,usize);n],
    };
    let div = 998244353;
    let mut dp = vec![(1,1)];
    let win = tup.windows(2);
    for (i,val) in win.enumerate(){
        let mut x=0;
        let mut y=0;
        if val[1].0 != val[0].0{
            x = dp[i].0
        }
        if val[1].0 != val[0].1{
            x += dp[i].1
        }
        x %= div;
        if val[1].1 != val[0].0{
            y = dp[i].0
        }
        if val[1].1 != val[0].1{
            y += dp[i].1
        }
        y %= div;
        dp.push((x,y));
    }
    println!("{}",(dp[n-1].0 + dp[n-1].1) % div);
}