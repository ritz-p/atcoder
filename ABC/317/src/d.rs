use proconio::input;

fn main(){
    input!{
        n: usize,
        p: [(i64,i64,usize);n]
    };
    let np = p.into_iter().map(|(x,y,z)| {
        let half = (x + y + 1) / 2;
        (0.max(half - x) as usize,z)
    }).collect::<Vec<_>>();
    let z_sum = np.iter().fold(0, |sum, &(_, z) | sum + z);
    let mut dp = vec![usize::MAX;z_sum+1];
    dp[0] = 0;

    for (d,z) in np{
        for now in (0..z_sum).rev(){
            if now + z > z_sum{
                continue;
            }
            if dp[now] == usize::MAX{
                continue;
            }
            dp[now+z] = dp[now+z].min(dp[now] + d);
        }
    }

    println!("{}",dp[(z_sum+1)/2..].into_iter().min().unwrap());
}
