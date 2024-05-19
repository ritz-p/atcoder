use proconio::input;

fn main(){
    input!{
        n: usize,
        arr: [(isize,isize);n],
    }
    for i in 0..n{
        println!("{}",arr[i].0+arr[i].1);
    }
}