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

cat << _EOF
## CS Academy

| Problem | Solution |
|:--------|:-------- |
_EOF

for file in `ls ./archives/csacademy/`
do
    problem_id=`echo ${file} | sed -e "s/\.rs$//g"`
    echo "| [${problem_id}](https://csacademy.com/contest/archive/task/${problem_id}) | [${file}](./archives/csacademy/${file}) |"
done
