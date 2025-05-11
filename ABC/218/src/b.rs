use proconio::input;

fn main() {
    input! {
        a: [usize;26]
    };
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();

    for e in a {
        print!("{}", alphabet[e - 1]);
    }
    println!();
}
