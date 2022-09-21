use crate::ChannelType;
use std::{fs::File, io::prelude::*, io::BufReader, io::Read, thread, time};

#[derive(Debug)]
pub struct MemUsage {
    Mem_Total: u32,
    Mem_Free: u32,
    Mem_Available: u32,
    Mem_Buffer: u32,
    Mem_Cached: u32,
}

impl MemUsage {
    fn new(line_vector: Vec<u32>) -> MemUsage {
        MemUsage {
            Mem_Total: line_vector[0],
            Mem_Free: line_vector[1],
            Mem_Available: line_vector[2],
            Mem_Buffer: line_vector[3],
            Mem_Cached: line_vector[4],
        }
    }
}

pub async fn main_mem_stat_handler(transmitter: std::sync::mpsc::Sender<ChannelType>) {
    loop {
        let procmeminfo_fd = File::open("/proc/meminfo").unwrap();
        let mut buff_reader = BufReader::new(&procmeminfo_fd);
        let mut meminfo = String::new();
        let _ = buff_reader.read_to_string(&mut meminfo);
        let lines: Vec<&str> = meminfo.split("\n").into_iter().collect();
        let mut temp_vec: Vec<u32> = Vec::new();
        //Can just label 0-4 itself instead of loop
        for i in 0..5 {
            let pos1 = lines[i].chars().position(|c| c == ':').unwrap() + 1;
            let pos2 = lines[i].chars().position(|c| c == 'k').unwrap();
            let finalvalue = lines[i].get(pos1..pos2).unwrap();
            let line_val = finalvalue.trim().parse::<u32>().unwrap();
            temp_vec.push(line_val);
        }
        let new_mem_usage = MemUsage::new(temp_vec);
        transmitter
            .send(ChannelType::MemData(new_mem_usage))
            .unwrap();

        thread::sleep(time::Duration::from_millis(100));
    }
}
