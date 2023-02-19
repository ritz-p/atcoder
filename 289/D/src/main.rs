use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        m: usize,
        mochi: [usize; m],
        x: usize,
    }
    let mut dp = vec![false; x + 1];
    let mut reachable = vec![true; x + 1];
    dp[0] = true;
    for i in mochi {
        reachable[i] = false;
    }

    for i in 0..x + 1 {
        if reachable[i] {
            for aa in a.clone() {
                if i >= aa {
                    //論理OR(どちらか 1 なら 1)
                    dp[i] = dp[i] || dp[i - aa];
                }
            }
        }
    }

    if dp[x] {
        println!("Yes");
    } else {
        println!("No");
    }
}
