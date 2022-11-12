#!/bin/bash 
cd ./SysModule 
cargo build --release
mv $PWD/target/release/SysModule .
cd ..
cd SysPerfTUI
go build .
cd ..
./SysModule/SysModule > /dev/null 2>&1 & 
./SysPerfTUI/SysPerfTUI
