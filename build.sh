#!/bin/bash 
cd ./SysModule 
cargo build --release
mv $PWD/target/release/SysModule .
cd ..
cd SysPerfTUI
go build .
cd ..

echo "Finished building binaries, use run.sh to run the program"
