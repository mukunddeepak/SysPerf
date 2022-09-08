
mod CPUStat;
mod MEMStat;

#[tokio::main]
async fn main() {

    //Do note, tokio uses single OS thread for all spwaned threads
    //Non Blocking spawns : 
    tokio::spawn(async{
        MEMStat::memfuncs::main_mem_stat_handler().await;
    });

    tokio::spawn(async{
        CPUStat::statfuncs::main_cpu_stat_handler().await;
    });

}
