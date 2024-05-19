use proconio::input;
use proconio::marker::Chars;
fn main(){
    input!{
        h: usize,
        w: usize,
        _n: usize,
        t: Chars,
        s: [Chars;h]
    };

    let mut res = 0;
    for i in 1..h{
        for j in 1..w{
            let mut current_y = i;
            let mut current_x = j;
            let mut f = true;
            if s[current_y][current_x] == '#'{
                continue;
            }
            for c in &t{
                match c{
                    'L' => {
                        current_x -= 1;
                    },
                    'R' => {
                        current_x += 1;
                    },
                    'U' => {
                        current_y -= 1;
                    },
                    'D' => {
                        current_y += 1;
                    },
                    _ => {}
                }
                if s[current_y][current_x] == '#'{
                    f = false;
                    break;
                }
            }
            if f{
                res += 1;
            }
        }
    }
    println!("{}",res);
}
