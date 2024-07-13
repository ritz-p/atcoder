if [ -z "${1}" ]; then
    echo "You should set contest number"
    exit 1
fi
mkdir -p ABC/$1
mkdir -p ABC/$1/src
touch ABC/$1/Cargo.toml
echo "[package]
name = \"abc_$1\"
version = \"0.1.0\"
authors = [\"root\"]
edition = \"2018\"

[dependencies]
proconio = \"0.4.5\"
itertools = \"0.10.5\"
ac-library-rs = \"0.1.1\"" > ABC/$1/Cargo.toml

alphabet=(A B C D E F G Ex)
for abc in ${alphabet[@]}; do
    lower_case=$(echo $abc | tr 'A-Z' 'a-z')
    touch ABC/$1/src/$lower_case.rs

    echo "
[[bin]]
name = \"$lower_case\"
path = \"src/$lower_case.rs\"" >> ABC/$1/Cargo.toml

    echo "use proconio::input;

fn main(){
    input!{

    };
}" > ABC/$1/src/$lower_case.rs
done

echo "[workspace]
members = [
    \"ABC/$1\",
    \"ironclad_rule\"
]" > Cargo.toml;
cargo fmt