use proconio::input;

fn main(){
    input!{
        n: usize,
        arr: [usize;n]
    };

    let mut res=0;
    for i in 0..n{
        if arr[res]<arr[i]{
            res=i;
        }
    }
    println!("{}",res+1);

}