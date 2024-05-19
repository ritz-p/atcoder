use proconio::input;

fn main(){
    input!{
        mut n: usize,
    };
    let mut current = 2;
    while n > 1{
        if n % current == 0{
            n = n/current;
        }else{
            current += 1;
        }
        if current > 3{
            break;
        }
    }
    if current > 3{
        println!("No");
    }else{
        println!("Yes");
    }
}
