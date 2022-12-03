use proconio::input;

fn main(){
    input!{
        n: usize,
        arr: [isize;n],
    };
    let mut s = arr[0];
    print!("{} ",s);
    for i in 1..n{
        if i < n-1{
            print!("{} ",arr[i]-s);
        }else{
            print!("{}",arr[i]-s);
        }
        s += arr[i]-s;
    }
}