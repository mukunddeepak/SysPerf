use std::{fs::File, io::prelude::*, io::BufReader,io::Read, thread, time};

// macro_rules! skip_fail {
//     ($res:expr) => {
//         match $res {
//             Ok(val) => 
//             {val}
//             ,
//             Err(_) => {   
//                 continue;
//             }
//         }
//     };
// }
#[derive(Debug)]
struct MemUsage {
    Mem_Total:u32,
    Mem_Free:u32,
    Mem_Available:u32,
    Mem_Buffer:u32,
    Mem_Cached:u32,
    // Mem_SwapCached:u32,
    // Mem_Active:u32,
    // Mem_Inactive:u32,
    // Mem_HighTotal:u32,
    // Mem_HighFree:u32,
    // Mem_LowTotal:u32,
    // Mem_LowFree:u32,
    // Mem_SwapTotal:u32,
    // Mem_SwapFree:u32,
    // Mem_Dirty:u32,
    // Mem_WriteBack:u32,
    // Mem_AnonPages:u32,
    // Mem_Mapped:u32,
    // Mem_Shmem:u32,
    // Mem_KReclaim:u32,
    // Mem_Slab:u32,
    // Mem_SReclaim:u32,
    // Mem_UnReclaim:u32,
    // Mem_PageTables:u32,
    // Mem_NFSUnstable:u32,
    // Mem_Bounce:u32,
    // Mem_WritebackTmp:u32,
    // Mem_CommitLimit:u32,
    // Mem_CommitedAS:u32,
    // Mem_VmallocTotal:u32,
    // Mem_VmallocUsed:u32,
    // Mem_VmallocChunk:u32,
    // Mem_Percep:u32,
    // Mem_HardwareCorrupt:u32,
    // Mem_AnonHugePages:u32,
    // Mem_ShMemHugePages:u32,
    // Mem_ShmemPmdMapped:u32,
}

impl MemUsage{
    fn new(line_vector: Vec<u32>) -> MemUsage{
        MemUsage{
            Mem_Total:line_vector[0],
            Mem_Free:line_vector[1],
            Mem_Available:line_vector[2],
            Mem_Buffer:line_vector[3],
            Mem_Cached:line_vector[4],
            // Mem_SwapCached:line_vector[5].trim().parse().unwrap(),
            // Mem_Active:line_vector[6].trim().parse().unwrap(),
            // Mem_Inactive:line_vector[7].trim().parse().unwrap(),
            // Mem_HighTotal:line_vector[8].trim().parse().unwrap(),
            // Mem_HighFree:line_vector[9].trim().parse().unwrap(),
            // Mem_LowTotal:line_vector[10].trim().parse().unwrap(),
            // Mem_LowFree:line_vector[11].trim().parse().unwrap(),
            // Mem_SwapTotal:line_vector[12].trim().parse().unwrap(),
            // Mem_SwapFree:line_vector[13].trim().parse().unwrap(),
            // Mem_Dirty:line_vector[14].trim().parse().unwrap(),
            // Mem_WriteBack:line_vector[15].trim().parse().unwrap(),
            // Mem_AnonPages:line_vector[16].trim().parse().unwrap(),
            // Mem_Mapped:line_vector[17].trim().parse().unwrap(),
            // Mem_Shmem:line_vector[18].trim().parse().unwrap(),
            // Mem_KReclaim:line_vector[19].trim().parse().unwrap(),
            // Mem_Slab:line_vector[20].trim().parse().unwrap(),
            // Mem_SReclaim:line_vector[21].trim().parse().unwrap(),
            // Mem_UnReclaim:line_vector[22].trim().parse().unwrap(),
            // Mem_PageTables:line_vector[23].trim().parse().unwrap(),
            // Mem_NFSUnstable:line_vector[24].trim().parse().unwrap(),
            // Mem_Bounce:line_vector[25].trim().parse().unwrap(),
            // Mem_WritebackTmp:line_vector[26].trim().parse().unwrap(),
            // Mem_CommitLimit:line_vector[27].trim().parse().unwrap(),
            // Mem_CommitedAS:line_vector[28].trim().parse().unwrap(),
            // Mem_VmallocTotal:line_vector[29].trim().parse().unwrap(),
            // Mem_VmallocUsed:line_vector[30].trim().parse().unwrap(),
            // Mem_VmallocChunk:line_vector[31].trim().parse().unwrap(),
            // Mem_Percep:line_vector[32].trim().parse().unwrap(),
            // Mem_HardwareCorrupt:line_vector[33].trim().parse().unwrap(),
            // Mem_AnonHugePages:line_vector[34].trim().parse().unwrap(),
            // Mem_ShMemHugePages:line_vector[35].trim().parse().unwrap(),
            // Mem_ShmemPmdMapped:line_vector[36].trim().parse().unwrap(),
        }
    }
}
//Have to use MemUsage for buffer




pub async fn main_mem_stat_handler(){
    let mut iteration=0;
    let mut timed_storage_buffer_1: Vec<MemUsage> = Vec::new();

    loop{
    println!("Iteration {} \n \n",iteration);
    let mut procmeminfo_fd = File::open("/proc/meminfo").unwrap();
    let mut buff_reader = BufReader::new(&procmeminfo_fd);
    let mut meminfo = String::new();
    let _ =buff_reader.read_to_string(&mut meminfo);
    let mut lines=meminfo.split("\n");
    let mut i=0;
    let  mut temp_vec : Vec<u32> = Vec::new();
    for line in lines {
        if(i!=5){
        let mut pos1=line.chars().position(|c| c == ':').unwrap()+1;
        let mut pos2=line.chars().position(|c| c == 'k').unwrap();
        let mut val_label=line.get(0..pos1-1).unwrap();
        i+=1;
        let mut finalvalue=line.get(pos1..pos2).unwrap();  
        let line_val = finalvalue.trim().parse::<u32>().unwrap();
        temp_vec.push(line_val);
        }
    }
    let new_mem_usage = MemUsage::new(temp_vec);
    timed_storage_buffer_1.push(new_mem_usage);
    if timed_storage_buffer_1.len() == 1 {
            continue;
        }

    println!("{:?}\n",timed_storage_buffer_1.last().unwrap());
    iteration+=1;
    
    thread::sleep(time::Duration::from_millis(1000));
}
}
