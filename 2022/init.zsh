#!/usr/bin/env zsh

local day=$1
[[ -n $day ]] || { echo '$1 is day'; return 1 }

local padded_day=$( python -c "print('$day'.zfill(2), end=None)" )
local folder="day${padded_day}-rust"

if [[ -d $folder ]]; then
  gum confirm "folder '$folder' exists, continue?" --default=No || return 0
fi

\rm -rf $folder
cargo init --lib $folder
echo "Created new Rust project in $folder ..."

cp Makefile_template $folder/Makefile
sed -ri "s|__aoc_url__|https://adventofcode.com/2022/day/$day/input|" $folder/Makefile

local existing_envrc=$( fd --hidden --no-ignore .envrc | head -n1 )
if [[ -n $existing_envrc ]]; then
  echo "copying existing .envrc from '$existing_envrc' ..."
  cp $existing_envrc $folder
else
  echo 'creating new .envrc ...'
  cp envrc_template $folder/.envrc
fi
direnv allow $folder/.envrc

cat /dev/null > $folder/src/lib.rs
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
