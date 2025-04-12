use proconio::input;
fn main() {
    input! {
        n: usize,
        s: [String;n]
    };
    let mut login = false;

    let mut res = 0;
    for c in s {
        match c.as_str() {
            "login" => {
                login = true;
            }
            "logout" => {
                login = false;
            }
            "private" => {
                if !login {
                    res += 1;
                }
            }
            "public" => {}
            _ => {}
        }
    }

    println!("{}", res);
}
