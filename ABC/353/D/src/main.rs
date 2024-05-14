use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let m = 998244353;

    let mut digits:Vec<usize> = vec![0;11];
    for i in 0..n{
        digits[a[i].to_string().len()] += 1;
    }
    let mut res = 0;
    for i in 0..n{
        for (index,k) in digits.iter().enumerate(){
            if *k == 0{
                continue;
            }
            let mut digit = 1;
            for _ in 0..index{
                digit *= 10;
            }
            let num = a[i] * digit % m;
            res += num * k % m;
            if index == a[i].to_string().len(){
                res -= (digit * a[i]) % m;
            }
        }
        digits[a[i].to_string().len()] -= 1;
        res += (a[i] * i) % m;
    }
    println!("{}", res % m);
}
