use crate::MemUsageProtobuf; //Imported protobuf

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
    pub fn new() -> MemUsage {
        MemUsage {
            Mem_Total: 0,
            Mem_Free: 0,
            Mem_Available: 0,
            Mem_Buffer: 0,
            Mem_Cached: 0,
        }
    }

    fn update_values(&mut self, line_vector: Vec<u32>) {
        self.Mem_Total = line_vector[0];
        self.Mem_Free = line_vector[1];
        self.Mem_Available = line_vector[2];
        self.Mem_Buffer = line_vector[3];
        self.Mem_Cached = line_vector[4];
    }

    pub fn convert_to_protobuf(&self) -> MemUsageProtobuf {
        println!("Hello world, {}", self.Mem_Total);
        MemUsageProtobuf {
            mem_total: self.Mem_Total,
            mem_free: self.Mem_Free,
            mem_available: self.Mem_Available,
            mem_buffer: self.Mem_Buffer,
            mem_cached: self.Mem_Cached,
        }
    }
}

pub async fn main_mem_stat_handler(statefull_mem_usage: &mut MemUsage) {
    println!("Jello");
    loop {
        let procmeminfo_fd = match File::open("/proc/meminfo") {
            Ok(x) => x,
            Err(_) => {
                panic!("Make sure you are using a linux system! Error point[reading stat file]");
            }
        };

        let mut buff_reader = BufReader::new(&procmeminfo_fd);
        let mut meminfo = String::new();
        let _ = buff_reader.read_to_string(&mut meminfo);

        let mut lines: Vec<&str> = meminfo.split("\n").into_iter().collect();
        let mut temp_vec: Vec<u32> = Vec::new();

        for i in 0..5 {
            let pos1 = lines[i].chars().position(|c| c == ':').unwrap() + 1;
            let pos2 = lines[i].chars().position(|c| c == 'k').unwrap();
            let finalvalue = lines[i].get(pos1..pos2).unwrap();
            let line_val = finalvalue.trim().parse::<u32>().unwrap();
            temp_vec.push(line_val);
        }
        statefull_mem_usage.update_values(temp_vec);
        thread::sleep(time::Duration::from_millis(100));
    }
}
