use proconio::input;
use std::cmp::max;

fn main() {
    input!{
        n: usize,
        m: usize,
        mut a: [usize;n]
    };
    a.sort();
    let mut res = 0;
    for i in 0..n {
        let idx = binary_search(&a, a[i] + m);
        let pre = idx - i;
        res = max(res, pre);
    }
    println!("{}", res);
}

fn binary_search(arr: &Vec<usize>, target: usize) -> usize {
    let mut left = 0;
    let mut right = arr.len() - 1;
    while left <= right {
        let mid = (left + right) / 2;
        if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    left
}