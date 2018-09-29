#!/bin/sh

cat << _EOF
# Rust solutions to coding problems 

## AtCoder

| Problem | Solution |
|:--------|:-------- |
_EOF

for directory in `ls ./archives/atcoder/`
do
    for file in `ls ./archives/atcoder/${directory}/`
    do
        problem_id=`echo ${file} | sed -e "s/\.rs$//g"`
        echo "| [${problem_id}](https://beta.atcoder.jp/contests/${directory}/tasks/${problem_id}) | [${file}](./archives/atcoder/${directory}/${file}) |"
    done
done