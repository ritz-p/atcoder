use proconio::input;

fn main(){
    input!{
        b: isize,
    };
    let mut a:isize = 1;
    loop{
        let check = a.checked_pow(a as u32);
        if check.is_some(){
            let current:isize = a.pow(a as u32);
            if current == b{
                println!("{}",a);
                return;
            }
        }else{
            println!("-1");
            return;
        }
        a += 1;
    }
}
