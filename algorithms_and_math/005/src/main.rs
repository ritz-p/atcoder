use proconio::input;

fn main(){
    input!{
        n: usize,
        arr: [usize;n],
    };
    let mut res=0;
    for i in 0..n{
        res += arr[i];
    }
    println!("{}",res % 100);
}