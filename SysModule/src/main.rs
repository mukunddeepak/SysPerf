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

use tonic::Request;
//Cargo crate modules :
use tonic::transport::Server;

// -------------Protobuf side constructs and GRPC's----------
pub mod protobuf {
    tonic::include_proto!("data.protobuf");
}

use protobuf::CpuUsage as CpuUsageProtobuf;
use protobuf::MemUsage as MemUsageProtobuf;
use protobuf::InitData as InitDataProtobuf;
use protobuf::{CpuUsageRequest, MemUsageRequest, EmptyReq};
//Server trait
use protobuf::fetch_data_server::{FetchData, FetchDataServer};
use protobuf::fetch_data_mem_server::{ FetchDataMem, FetchDataMemServer};

#[tonic::async_trait]
impl FetchData for MultiCpuUsage {
    async fn init_cpu_detail(
        &self,
        req: tonic::Request<EmptyReq>,
        ) -> std::result::Result<tonic::Response<InitDataProtobuf>, tonic::Status> {
        return Ok(tonic::Response::new(self.convert_to_detail_protobuf()))
    }    
    async fn fetch_cpu_usage(
        &self,
        req: tonic::Request<CpuUsageRequest>,
        ) -> std::result::Result<tonic::Response<CpuUsageProtobuf>, tonic::Status> {
        let request_paylod = req.into_inner();
        return Ok(tonic::Response::new(self.convert_to_protobuf(request_paylod)));
    }    
}

#[tonic::async_trait]
impl FetchDataMem for MemUsage{
    async fn fetch_mem_usage(
        &self,
        req: tonic::Request<MemUsageRequest>,
        ) -> std::result::Result<tonic::Response<MemUsageProtobuf>, tonic::Status> {
        return Ok(tonic::Response::new(self.convert_to_protobuf()));
    }
}

//Rust side structs
use CPUStat::statfuncs::MultiCpuUsage;
use MEMStat::memfuncs::MemUsage;

#[tokio::main]
async fn main() -> Result<()> {
    //Do note, tokio uses single OS thread for all spawned threads
    //Non Blocking spawns :

    /*     tokio::spawn(async {
           MEMStat::memfuncs::main_mem_stat_handler(mem_tx).await;
           });
           */
    let mut statefull_cpu_usage = MultiCpuUsage::new();
    let mut arc_statefull_cpu_usage = Arc::new(statefull_cpu_usage);
    let clone = Arc::clone(&arc_statefull_cpu_usage);

    let mut statefull_mem_usage = MemUsage::new();
    let mut arc_statefull_mem_usage = Arc:: new(statefull_mem_usage);
    let clone1=Arc::clone(&arc_statefull_mem_usage);


    tokio::spawn(async move{
        let addr = "[::1]:5001".parse().unwrap();
        println!("Listening on port 5001");
        Server::builder()
            .add_service(FetchDataServer::from_arc(clone))
            .add_service(FetchDataMemServer::from_arc(clone1))
            .serve(addr)
            .await;
    });
    unsafe{
        tokio::spawn(async move{
            CPUStat::statfuncs::main_cpu_stat_handler(&mut Arc::get_mut_unchecked(&mut arc_statefull_cpu_usage)).await;
        });
        tokio::spawn(async move{
            MEMStat::memfuncs::main_mem_stat_handler(&mut Arc::get_mut_unchecked(&mut arc_statefull_mem_usage)).await;
        }).await;
        //Last await is to keep the main thread alive in the aysnc space.
    }


    Ok(())
}
