use proconio::input;

fn main(){
    input!{
        x: isize,
        y: isize
    };

    if (x - y <= 3 && x - y > 0) || (y - x <= 2 && y - x > 0){
        println!("Yes");
    }else{
        println!("No");
    }
}
