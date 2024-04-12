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
    if dp[n][s]{
        println!("Yes");
    }else{
        println!("No");
    }
}