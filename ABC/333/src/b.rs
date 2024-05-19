use proconio::input;
fn main(){
    input!{
        s: String,
        t: String,
    };
    if s == "AB" || s == "BC" || s == "CD" || s == "DE" || s == "EA" || s == "AE" || s == "ED" || s == "DC" || s == "CB" || s == "BA"{
        if t == "AB" || t == "BC" || t == "CD" || t == "DE" || t == "EA" || t == "AE" || t == "ED" || t == "DC" || t == "CB" || t == "BA"{
            println!("Yes");
        }else{
            println!("No");
        }
    }else if s == "AC" || s == "AD" || s == "BD" || s == "BE" || s == "CE" || s == "CA" || s == "DA" || s == "DB" || s == "EB" || s == "EC"{
        if t == "AC" || t == "AD" || t == "BD" || t == "BE" || t == "CE" || t == "CA" || t == "DA" || t == "DB" || t == "EB" || t == "EC"{
            println!("Yes");
        }else{
            println!("No");
        }
    }

}
