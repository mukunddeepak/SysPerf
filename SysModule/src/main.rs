//Linter controls
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused)]
#![feature(get_mut_unchecked)]

//Local module system :
mod CPUStat;
mod MEMStat;

//std imports
use std::io::Result;
use std::sync::mpsc;
use std::sync::Arc;

//Cargo crate modules :
use tonic::transport::Server;

// -------------Protobuf side constructs and GRPC's----------
pub mod protobuf {
    tonic::include_proto!("data.protobuf");
}

use protobuf::CpuUsage as CpuUsageProtobuf;
use protobuf::MemUsage as MemUsageProtobuf;
use protobuf::{CpuUsageRequest, MemUsageRequest};
//Server trait
use protobuf::fetch_data_server::{FetchData, FetchDataServer, FetchDataMem, FetchDataMemServer};

#[tonic::async_trait]
impl FetchData for CpuUsage {
    async fn fetch_cpu_usage(
        &self,
        req: tonic::Request<CpuUsageRequest>,
        ) -> std::result::Result<tonic::Response<CpuUsageProtobuf>, tonic::Status> {
        return Ok(tonic::Response::new(self.convert_to_protobuf()));
    }    
}

impl FetchDataMem for MemUsage{
 async fn fetch_mem_usage(
        &self,
       req: tonic::Request<MemUsageRequest>,
      ) -> std::result::Result<tonic::Response<MemUsageProtobuf>, tonic::Status> {
     return Ok(tonic::Response::new(self.convert_to_protobuf()));
    }
}



//Rust side structs
use CPUStat::statfuncs::CpuUsage;
use MEMStat::memfuncs::MemUsage;

#[tokio::main]
async fn main() -> Result<()> {
    //Do note, tokio uses single OS thread for all spwaned threads
    //Non Blocking spawns :

    /*     tokio::spawn(async {
           MEMStat::memfuncs::main_mem_stat_handler(mem_tx).await;
           });
           */
    let mut statefull_cpu_usage = CpuUsage::new();
    let mut arc_statefull_cpu_usage = Arc::new(statefull_cpu_usage);
    let clone = Arc::clone(&arc_statefull_cpu_usage);


    tokio::spawn(async move{
        let addr = "[::1]:5001".parse().unwrap();
        println!("Listening on port 5001");
        Server::builder()
            .add_service(FetchDataServer::from_arc(clone))
            .add_service(FetchDataMemServer)
            .serve(addr)
            .await;
    });
    unsafe{
        CPUStat::statfuncs::main_cpu_stat_handler(&mut Arc::get_mut_unchecked(&mut arc_statefull_cpu_usage));
    }


    Ok(())
 }
