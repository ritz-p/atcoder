use proconio::input;
fn main() {
    input! {
        n: usize,
        a: [usize;n]
    };
    let mut sum = vec![0; n];
    let mut cums = vec![0; n];
    let mut max = vec![0; n];

    for i in 0..n {
        if i > 0 {
            sum[i] = sum[i - 1];
            max[i] = max[i - 1];
            cums[i] = cums[i - 1];
        }

        sum[i] += a[i];
        max[i] = max[i].max(a[i]);
        cums[i] += sum[i];
    }

    for i in 0..n {
        let res = max[i] * (i + 1) + cums[i];
        println!("{}", res);
    }
}
