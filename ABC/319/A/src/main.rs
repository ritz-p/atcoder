use proconio::input;

fn main(){
    input!{
        s: String
    };
    let mut v: Vec<(String,usize)> = vec![("tourist".to_string(),3858),
    ("ksun48".to_string(),3679),
    ("Benq".to_string(),3658),
    ("Um_nik".to_string(),3648),
    ("apiad".to_string(),3638),
    ("Stonefeang".to_string(),3630),
    ("ecnerwala".to_string(),3613),
    ("mnbvmar".to_string(),3555),
    ("newbiedmy".to_string(),3516),
    ("semiexp".to_string(),3481)];

    for (x,y) in v{
        if x == s{
            println!("{}",y);
            return;
        }
    }
}
