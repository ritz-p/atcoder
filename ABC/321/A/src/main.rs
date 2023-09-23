use proconio::input;
fn main(){
    input!{
        n: String,
    };

    let length = n.chars().count();
    if length == 1 {
        println!("Yes");
        return;
    }

    for i in 0..length - 1 {
        if n.chars().nth(i).unwrap().to_digit(10).unwrap() <= n.chars().nth(i + 1).unwrap().to_digit(10).unwrap() {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
