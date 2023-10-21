use proconio::input;
fn main() {
    input!{
        n: usize,
        l: usize,
        k: usize,
        a: [usize;n]
    }
    let mut left:isize = -1;
    let mut right:isize = l as isize + 1;

    while right - left > 1{
        let mid = (left + right) / 2;
        if check(n,l,k,&a,mid as usize){
            left = mid;
        }else{
            right = mid;
        }
    }

    println!("{}",left);
}


fn check(n:usize,l:usize,k:usize,a:&Vec<usize>,x: usize) -> bool{
    let mut num = 0;
    let mut pre = 0;
    for i in 0..n{
        if a[i] - pre >= x{
            num += 1;
            pre = a[i];
        }
    }

    if l - pre >= x{
        num += 1;
    }
    return num >= k+1;
}

