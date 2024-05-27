use proconio::input;

fn main(){
    input!{
        n: usize,
        m: usize,
        q: usize,
        lr: [(usize,usize);m],
        pq: [(usize,usize);q]
    };
    let mut sums = vec![vec![0;n+1];n+1];
    //二次元の累積和
    for (l,r) in lr{
        sums[l][r] += 1;
    }

    for i in 1..=n{
        for j in 1..n{
            sums[i][j+1] += sums[i][j];
        }
    }

    for (p,q) in pq{
        let mut res = 0;
        for i in p..=q{
            res += sums[i][q];
        }
        println!("{}",res);
    }
}
