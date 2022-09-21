//Linter controls
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused)]

//Module system : 
mod CPUStat;
mod MEMStat;

//std imports
use std::sync::mpsc;
use std::io::Result;

// Protobuf side constructs
pub mod items{
    include!(concat!(env!("OUT_DIR"), "/data.datastruct.rs"));
}

use items::MemUsage as MemUsage_protobuf;
use items::CpuUsage as CpuUsage_protobuf;
use items::Data;
use items::data::DataContent;
use items::data::Type;

impl Data{
    fn new()-> Data{
        Data{
            r#type : Type::Cpu as i32, //Protobuf implements conversion to i32 traits
            data_content : Some(DataContent::CpuData(CpuUsage_protobuf{
                cpu_id : 0,
                cpu_usage : 23
            }))
        }
    }
}

//Rust side structs
use CPUStat::statfuncs::CpuUsage;
use MEMStat::memfuncs::MemUsage;

#[derive(Debug)]
pub enum ChannelType {
    MemData(MemUsage),
    CpuData(CpuUsage),
}

#[tokio::main]
async fn main() -> Result<()> {
    //Do note, tokio uses single OS thread for all spwaned threads
    //Non Blocking spawns :
    let (comm_tx, comm_rx) = mpsc::channel();
    let cpu_tx = comm_tx.clone();
    let mem_tx = comm_tx.clone();
    tokio::spawn(async {
        MEMStat::memfuncs::main_mem_stat_handler(mem_tx).await;
    });

    tokio::spawn(async {
        CPUStat::statfuncs::main_cpu_stat_handler(cpu_tx).await;
    });
    let buffer_struct = Data::new();
    println!("{:?}", buffer_struct);
    Ok(())

        /*  loop {
            println!("{:?}", comm_rx.recv().unwrap());
            } */

}
