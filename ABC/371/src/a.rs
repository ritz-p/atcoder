use proconio::input;

fn main() {
    input! {
        ab: char,
        ac: char,
        bc: char
    };
    if ab == '>' {
        if ac == '>' {
            if bc == '>' {
                println!("B");
            } else {
                println!("C");
            }
        } else {
            if bc == '<' {
                println!("A");
            }
        }
    } else {
        if ac == '>' {
            if bc == '>' {
                println!("A");
            }
        } else {
            if bc == '>' {
                println!("C");
            } else {
                println!("B");
            }
        }
    }
}
