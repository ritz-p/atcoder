use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut v: [(usize, usize); m]
    }
    v.sort();
    // 最大が頂点の数のため
    let mut belongs = Vec::with_capacity(n);
    for i in 0..n {
        belongs.push(i);
    }

    for (mut a, mut b) in v {
        if a > b {
            let tmp = a;
            a = b;
            b = tmp;
        }
        let new = belongs[a-1];
        let old = belongs[b-1];
        for belong in belongs.iter_mut() {
            if *belong == old {
                *belong = new;
            }
        }            
    }

    belongs.sort();
    // かぶりを削除
    belongs.dedup();
    println!("{}", belongs.len());
}
