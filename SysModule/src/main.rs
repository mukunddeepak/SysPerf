#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused)]
use std::sync::mpsc;
mod CPUStat;
mod MEMStat;

use CPUStat::statfuncs::CpuUsage;
use MEMStat::memfuncs::MemUsage;

#[derive(Debug)]
pub enum ChannelType{
    MemData(MemUsage),
    CpuData(CpuUsage)
}


#[tokio::main]
async fn main() {

    //Do note, tokio uses single OS thread for all spwaned threads
    //Non Blocking spawns : 
    let (comm_tx, comm_rx) = mpsc::channel();
    let cpu_tx = comm_tx.clone();
    let mem_tx = comm_tx.clone();
    tokio::spawn(async{
        MEMStat::memfuncs::main_mem_stat_handler(mem_tx).await;
    });

    tokio::spawn(async{
        CPUStat::statfuncs::main_cpu_stat_handler(cpu_tx).await;
    });
    loop{
        println!("{:?}", comm_rx.recv().unwrap());
    }
}
