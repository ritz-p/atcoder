use proconio::input;

fn main() {
    input!{
        n: usize,
        k: usize,
        mut arr: [usize;n],
    };
    let mut flag = vec![false;k+1];
    for i in 0..n{
        if arr[i] >= 0 && arr[i] < k{
            flag[arr[i]] = true;
        }
    }
    let mut max = k-1;
    for i in 0..=k+1{
        if !flag[i]{
            println!("{}",i);
            break
        }
    }
}
