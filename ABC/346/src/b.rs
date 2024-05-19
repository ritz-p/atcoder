use proconio::input;
fn main(){
    input!{
        w: usize,
        b: usize,
    };
    let s = "wbwbwwbwbwbw";
    let mut ss = String::from("");
    while ss.len() < 200{
        ss.push_str(s);
    }
    let mut wc = 0;
    let mut bc = 0;
    for i in 0..ss.len()-w-b{
        let ch = &ss[i..i+w+b];
        for c in ch.chars(){
            if c == 'b'{
                bc += 1;
            }else{
                wc += 1;
            }
            if wc > w || bc > b{
                break;
            }
        }
        if wc == w && bc == b{
            println!("Yes");
            return;
        }
        bc = 0;
        wc = 0;
    }
    println!("No");
}
