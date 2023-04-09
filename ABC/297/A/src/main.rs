use proconio::input;
    
fn main(){
    input!{
        n: usize,
        t: usize,
        arr: [usize;n],
    };
    for i in 0..n-1{
        if arr[i+1] - arr[i] <= t{
            println!("{}",arr[i+1]);
            return;
        }
    }
    println!("-1");
}
