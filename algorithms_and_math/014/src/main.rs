use proconio::input;

fn main(){
    input!{
        mut n: usize,
    };
    let mut arr = Vec::new();
    let mut prime = 2;
    while prime * prime <= n{
        if n % prime==0{
            n/=prime;
            arr.push(prime);
        }else{
            prime += 1;
        }
    }
    if n > 2{
        arr.push(n);
    }
    for i in 0..arr.len(){
        print!("{} ",arr[i]);
    }
}