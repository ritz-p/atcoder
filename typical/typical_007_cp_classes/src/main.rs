use proconio::input;
use std::cmp::min;
fn main() {
    input!{
        n: usize,
        mut a: [usize;n],
        mut q: usize,
    }
    a.sort();
    while q > 0{
        input!{
            b: usize
        };
        let pos = bisect_left(&a,b);
        let res = if pos == n{
            b - a[n-1]
        }else if pos == 0{
            a[0] - b
        }else{
            min(a[pos] - b,b - a[pos-1])
        };
        println!("{}",res);
        q -= 1;
    }
}

fn bisect_left(vec: &Vec<usize>,v: usize) -> usize{
    let n = vec.len();
    let mut left = 0;
    let mut right = n;
    if v < vec[0]{
        return 0;
    }
    while right - left > 1{
        let mid = (left + right) / 2;
        if vec[mid] <= v{
            left = mid;
        }else{
            right = mid;
        }
    }
    right
}