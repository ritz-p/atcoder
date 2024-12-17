use proconio::input;

fn main() {
    input! {
        n: usize
    };

    let mut res = vec![1];
    if n == 1 {
        println!("1");
        return;
    }
    for _i in 2..=n {
        let mut cp = res.clone();
        let l = cp.len();
        cp[l - 1] += 1;
        res.extend(cp);
    }
    let l = res.len();
    for i in 0..l {
        print!("{} ", res[i]);
    }
    for i in (0..l - 1).rev() {
        print!("{} ", res[i]);
    }
    println!();
}
