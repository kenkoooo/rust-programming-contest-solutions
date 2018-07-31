#!/bin/sh

BIN_FILE_NAME=target/release/${1}
SH_FILE_NAME=target/release/${1}.sh

cargo rustc --bin ${1} --release -- -C opt-level=s -C lto -C link-args=-Wl,-x,-S -C panic=abort
strip ${BIN_FILE_NAME}
upx ${BIN_FILE_NAME}
cat src/bin/${1}.rs | sed -e 's/^/# /g' > ${SH_FILE_NAME}
echo "cat << '_EOF' | base64 -d > /tmp/exec" >> ${SH_FILE_NAME}
base64 ${BIN_FILE_NAME} >> ${SH_FILE_NAME}
echo "_EOF" >> ${SH_FILE_NAME}
echo "chmod +x /tmp/exec" >> ${SH_FILE_NAME}
echo "/tmp/exec" >> ${SH_FILE_NAME}
chmod +x ${SH_FILE_NAME}
