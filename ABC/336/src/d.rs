use proconio::input;

fn main() {
    input! {
        n : usize,
        a : [usize; n]
    }

    let mut la = vec![1; n];
    let mut ra = vec![1; n];

    for i in 0..n {
        if i > 0 { la[i] = a[i].min(la[i - 1] + 1); }
    }
    for i in (0..n).rev() {
        if i < n - 1 { ra[i] = a[i].min(ra[i + 1] + 1); }
    }
    let mut ans = 0;
    for i in 0..n {
        ans = ans.max(la[i].min(ra[i]));
    }
    
    println!("{}", ans);
}