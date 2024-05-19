use proconio::input;

fn main(){
    input!{
        n: usize
    };
    if n <= 999{
        println!("{}",n);
    }else if n >= 1000 && n < 10000{
        println!("{}",n-n%10);
    }else if n >= 10000 && n < 100000{
        println!("{}",n-n%100);
    }else if n >= 100000 && n < 1000000{
        println!("{}",n-n%1000);
    }else if n >= 1000000 && n < 10000000{
        println!("{}",n-n%10000);
    }else if n >= 10000000 && n < 100000000{
        println!("{}",n-n%100000);
    }else if n >= 100000000 && n < 1000000000{
        println!("{}",n-n%1000000);
    }
}
