use proconio::input;

fn main(){
    input!{
        n: usize,
        s: usize,
        a: [usize;n]
    };
    let mut dp = vec![vec![false;s+1];n+1];
    dp[0][0] = true;

    for i in 1..=n{
        for j in 0..=s{
            if dp[i-1][j]{
                dp[i][j] = true;
                if j + a[i-1] <= s{
                    dp[i][j+a[i-1]] = true;
                }
            }
        }
    }
    if !dp[n][s]{
        println!("-1");
        return;
    }
    let mut res = vec![];
    let mut p = s;
    for i in (0..n).rev(){
        if !dp[i][p]{
            res.push(i + 1);
            p -= a[i];
        }
    }
    println!("{}",res.len());
    println!("{}",res.iter().rev().map(|e|e.to_string()).collect::<Vec<_>>().join(" "));
}