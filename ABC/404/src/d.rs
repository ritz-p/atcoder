use proconio::input;
fn main() {
    input! {
        n: usize,
        m: usize,
        c: [usize;n],
    };
    let mut zoo = vec![vec![]; m];
    for i in 0..m {
        input! {
            k: usize,
            a: [usize;k]
        };

        for e in a {
            zoo[i].push(e - 1);
        }
    }

    let mut res = usize::MAX;
    for bits in 0..3_usize.pow(n as u32) {
        let mut temp = bits;
        let mut visited = vec![0; n];
        let mut cost = 0;
        for i in 0..n {
            let count = temp % 3;
            visited[i] = count;
            cost += c[i] * count;
            temp /= 3;
        }
        let mut f = true;
        for i in 0..m {
            let mut count = 0;
            for &z in &zoo[i] {
                count += visited[z];
            }
            if count < 2 {
                f = false;
                break;
            }
        }

        if f {
            res = res.min(cost);
        }
    }
    println!("{}", res);
}
