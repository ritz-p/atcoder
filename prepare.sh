mkdir $1

alphabet=(A B C D E F G Ex)
for abc in ${alphabet[@]}
do
    mkdir -p $1/$abc/src
    touch $1/$abc/Cargo.toml
    touch $1/$abc/src/main.rs
    echo "[package]
name = \"$1_${abc,}\"
version = \"0.1.0\"
authors = [\"root\"]
edition = \"2018\"

[dependencies]
proconio = \"0.4.3\"
itertools = \"0.10.5\"" > $1/$abc/Cargo.toml
    echo "use proconio::input;
    
fn main(){
    input!{

    };
}" > $1/$abc/src/main.rs
done