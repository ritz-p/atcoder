use proconio::input;

fn main(){
    input!{
        n: usize,
    }
    println!("2");
    for i in 3..=n{
        if is_prime(i){
            println!("{}",i);
        }
    }
}

fn is_prime(n: usize) -> bool{
    let mut current = 2;
    while current * current <= n{
        if n % current == 0{
            return false;
        }
        current += 1;
    }
    true
}