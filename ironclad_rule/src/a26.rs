use proconio::input;

fn main(){
    input!{
        q: usize,
        x: [usize;q]
    }
    for n in x{
        if is_prime(n){
            println!("Yes");
        }else{
            println!("No");
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