use proconio::input;

fn main(){
    input!{
        n: usize,
        mut t: usize,
        arr: [usize;n],
    };
    let mut total = 0;
    for i in 0..n{
        total += arr[i];
    }
    t %= total;
    for i in 0..n{
        if t > arr[i]{
            t -= arr[i];
        }else{
            println!("{} {}",i+1,t);
            break;
        }
    }
}