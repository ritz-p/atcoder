use proconio::input;

fn main(){
    input!{
        n: usize,
        arr: [String;n],
    };
    for i in 0..n {
        println!("{}",arr[n-1-i]);
    }
}