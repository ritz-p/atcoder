use proconio::input;

fn main(){
    input!{
        s: String,
        t: String,
    };
    let mut res = false;
    let mut end = false;
    if (s.len() as usize) < (t.len() as usize){
        println!("No");
        return;
    }
    for i in 0..s.len() as usize - t.len() as usize + 1{
        if !end{
            for j in 0..t.len() as usize {
                if s.chars().nth(i+j).unwrap() == t.chars().nth(j).unwrap(){
                    res = true;
                    if j == t.len()-1{
                        end = true;
                    }
                }else{
                    res = false;
                    break;
                }
            }
        }
        if end{
            break;
        }
    }
    if res {
        println!("Yes");
    }else{
        println!("No");
    }
}
