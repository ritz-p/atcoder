use proconio::input;

fn main(){
    input!{
        n: usize,
        a: [usize;n]
    };
    let mut xor = a[0];
    for i in 1..n{
        xor = xor ^ a[i];
    }
    if xor != 0{
        println!("First");
    }else{
        println!("Second");
    }
}