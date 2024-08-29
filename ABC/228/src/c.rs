use proconio::input;
fn main() {
    input! {
        n: usize,
        k: usize,
        p: [(usize,usize,usize);n]
    };
    let mut sum = vec![];
    for (p1, p2, p3) in p {
        sum.push(p1 + p2 + p3);
    }
    let mut sum2 = sum.clone();
    sum2.sort();
    let border = sum2[n - k];
    for i in 0..n {
        if sum[i] + 300 >= border {
            println!("Yes")
        } else {
            println!("No")
        }
    }
}
