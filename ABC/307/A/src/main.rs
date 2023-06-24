use proconio::input;

fn main(){
    input!{
        n: usize,
        arr: [usize;n*7],
    };
    let mut t = 0;
    for i in 1..=n*7{
        t += arr[i-1];
        if i % 7 == 0{
            print!("{} ",t);
            t = 0;
        }
    }
}
