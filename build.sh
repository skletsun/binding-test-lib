#!/usr/bin/env bash

# Build java code first.
rm -fR java/target
mkdir java/target
javac -d java/target java/src/*.java

# Prepare environment variables for building and running tests.
# This requires executables and libraries to be able to find libjvm and Rust stdlib.
export JAVA_HOME="$(java -XshowSettings:properties -version 2>&1 > /dev/null | grep 'java.home' | awk '{print $3}')"
echo "JAVA_HOME=${JAVA_HOME}"

# Find the directory containing Rust libstd.
export RUST_LIB_DIR=$(rustup run stable rustc --print sysroot)/lib
echo "RUST_LIB_DIR: ${RUST_LIB_DIR}"

NEW_RUSTFLAGS="-C link-arg=-Wl,-rpath,${RUST_LIB_DIR}"

if [[ "${RUSTFLAGS:-}" != "" && "${RUSTFLAGS}" != "${NEW_RUSTFLAGS}" ]]; then
    echo "[WARNING]: RUSTFLAGS variable is set and will be overridden."
    echo "Set RUSTFLAGS=${RUSTFLAGS}"
    echo "New RUSTFLAGS=${NEW_RUSTFLAGS}"
fi
export RUSTFLAGS="${NEW_RUSTFLAGS}"
echo "RUSTFLAGS=${RUSTFLAGS}"

cargo build --all

# And execute...
java -Djava.library.path=target/debug -cp java/target Main
