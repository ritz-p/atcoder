use proconio::input;
    
fn main(){
    input!{
        arr: [String;8],
    };
    let abc = "abcdefgh";
    for i in 0..8{
        for j in 0..8{
            if arr[i].chars().nth(j).unwrap() == '*'{
                println!("{}{}",abc.chars().nth(j).unwrap(),8-i);
                break;
            }
        }
    }
}
