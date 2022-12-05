#!/usr/bin/env zsh

local day=$1
[[ -n $day ]] || { echo '$1 is day'; return 1 }

local padded_day=$( python -c "print('$day'.zfill(2), end=None)" )
local folder="day${padded_day}-rust"
\rm -rf $folder
cargo init --lib $folder
echo "Created new Rust project in $folder ..."

cp Makefile_template $folder/Makefile
sed -ri "s|__aoc_url__|https://adventofcode.com/2022/day/$day/input|" $folder/Makefile
cp envrc_template $folder/.envrc
direnv allow $folder/.envrc

mkdir -p $folder/src/bin
for p in {1..2}; do
  cat <<EOF >$folder/src/bin/p${p}.rs
fn main() {
    println!("+++++++++++++++++++ PART $p +++++++++++++++++++");
    let result = 0;

    // Let's do this!

    println!("result: {}", result);
    println!("------------------- PART $p -------------------");
}
EOF
done
