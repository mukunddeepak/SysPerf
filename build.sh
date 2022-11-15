#!/bin/bash 
sudo cp ./sysperf.service /etc/systemd/system/sysperf.service 
cd ./SysModule 
cargo build --release
sudo mv $PWD/target/release/SysModule /bin
cd ..
cd SysPerfTUI
go build -o sysperf .
sudo mv sysperf /bin
cd ..

echo "Start the backend service using systemctl start sysperf and later do sysperf anywhere!"

