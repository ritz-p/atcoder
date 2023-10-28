use proconio::input;

fn main(){
    input!{
        n: usize
    };
    let mut res = n;
    loop{
        let x = res / 100;
        let y = (res - x * 100) / 10;
        let z = res - x * 100 - y * 10;
        if x * y == z{
            println!("{}",res);
            break;
        }
        res += 1;
    }

}
