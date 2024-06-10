if [ -z "${1}" ]; then
    echo "You should set contest number"
    exit 1
fi
mkdir -p ADT/$1

alphabet=(A B C D E F G Ex)
for abc in ${alphabet[@]}
do
    mkdir -p ADT/$1/$abc/src
    touch ADT/$1/$abc/Cargo.toml
    touch ADT/$1/$abc/src/main.rs
    echo "[package]
name = \"${abc,}_$1\"
version = \"0.1.0\"
authors = [\"root\"]
edition = \"2018\"

[dependencies]
proconio = \"0.4.5\"
itertools = \"0.10.5\"
ac-library-rs = \"0.1.1\"" > ADT/$1/$abc/Cargo.toml
    echo "use proconio::input;

fn main(){
    input!{

    };
}" > ADT/$1/$abc/src/main.rs
done

echo "[workspace]
members = [
    \"ADT/$1/*\",
    \"ironclad_rule\"
]" > Cargo.toml;