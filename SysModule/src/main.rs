
mod CPUStat;
mod MEMStat;

#[tokio::main]
async fn main() {
    //Non Blocking spawns : 
    tokio::spawn(async{
        MEMStat::memfuncs::main_mem_stat_handler().await;
    });

    tokio::spawn(async{
        CPUStat::statfuncs::main_cpu_stat_handler().await;
    });

    // CPUStat::statfuncs::main_cpu_stat_handler();
}
