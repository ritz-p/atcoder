use proconio::input;

fn main(){
    input!{
        n: usize,
        q: [isize;n],
        a: [isize;n],
        b: [isize;n]
    };
    const INF: isize = 1_000_000_000_000_000_000;
    let mut ans = 0;
    for x in 0..=*q.iter().max().unwrap() {
        let mut y = INF;

        for i in 0..n{
            if q[i] < a[i] * x {
                y = -INF;
            } else if b[i] > 0 {
                y = y.min((q[i] - a[i] * x) / b[i]);
            }
        }

        ans = ans.max(x+y);
    }

    println!("{}",ans);
}
