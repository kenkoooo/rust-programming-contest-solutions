#!/bin/sh

if [ "$#" -ne 2 ]; then
    echo "Usage: ./add_bin.sh [JUDGE] [PROBLEM ID]"
    exit 1
fi

judge="$1"
problem_id="$2"
path="${judge}/${problem_id}.rs"

if [ -e ${path} ]
then
    echo "${path} is already exists"
else
    echo "[[bin]]" >> Cargo.toml
    echo "name = \"${problem_id}\"" >> Cargo.toml
    echo "path = \"${path}\"" >> Cargo.toml
fi

# Create a file
echo "
fn main() {
    let mut sc = Scanner::new();

}
" > ${path}
cat common/scanner.rs >> ${path}
git add ${path}

# Open in VSCode
code ./ ${path}
