#!/bin/sh

problem_id="$1"
path="src/bin/${problem_id}.rs"

# Check archives
EXISTING=`find ./archives/ | grep -F ${problem_id}.rs | head -n 1`
if [ -n ${EXISTING} ];then
    echo "${problem_id}.rs is already existing in ${EXISTING}"
    rm ${EXISTING}
    echo "removed"
else
    echo "${problem_id}.rs is not existing in archives."
fi


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
