use proconio::input;

fn main(){
    input!{
        n: usize,
        k: usize,
        a: [usize;n]
    };
    let mut count = 0;
    let mut current = 0;


    for e in a{
        if k >= current + e{
            current += e;
        }else{
            count += 1;
            current = e;
        }
    }
    println!("{}",count+1);
}
