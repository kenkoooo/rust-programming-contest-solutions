#!/bin/sh

problem_id="$1"
path="src/bin/${problem_id}.rs"

# Create a file
echo "
fn main() {
    let mut sc = Scanner::new();

}
" > ${path}
cat common/scanner.rs >> ${path}
git add ${path}

# Open in IntellJ
code ./ ${path}
