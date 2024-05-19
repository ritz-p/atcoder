use proconio::input;

fn main(){
    input!{
        n: usize,
        arr: [usize;n]
    };
    for i in arr{
        if i % 2 == 0{
            print!("{} ",i);
        }
    }
}