use proconio::input;
fn main(){
    input!{
        n: usize,
        m: usize,
        k: usize,
    };
    let gcd = gcd(n,m);
    let x = (n * m) / gcd;
    let mut left = 0;
    let mut right = usize::MAX;
    while left + 1 < right{
        let mid = (left + right) / 2;
        let y = (mid / n) + (mid / m) - 2 * (mid / x);
        if y < k{
            left = mid;
        }else{
            right = mid;
        }
    }
    println!("{}",right);
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}
