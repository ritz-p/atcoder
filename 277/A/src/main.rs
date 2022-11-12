use proconio::input;

fn main(){
    input!{
        n:usize,
        x:usize,
        arr:[usize;n],
    };
    let mut res=1;
    for i in 0..n{
        if arr[i] == x{
            res = i+1;
            break;
        }
    }
    println!("{}",res);
}