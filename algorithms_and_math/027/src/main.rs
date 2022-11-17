use proconio::input;

fn main(){
    input!{
        n:usize,
        mut arr:[usize;n],
    };
    arr.sort();
    for i in 0..n{
        if i == n-1{
            print!("{}",arr[i]);
        }else{
            print!("{} ",arr[i]);
        }
    }
}