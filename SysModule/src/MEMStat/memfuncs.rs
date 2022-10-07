use crate::{MemUsageProtobuf}; //Imported protobuf

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
            Mem_Total:0,
            Mem_Free: 0,
            Mem_Available: 0,
            Mem_Buffer: 0,
            Mem_Cached: 0,
        }
    }

    fn update_values(&mut self,line_vector: Vec<u32>) {
    
            self.Mem_Total= line_vector[0];
            self.Mem_Free= line_vector[1];
            self.Mem_Available= line_vector[2];
            self.Mem_Buffer=line_vector[3];
            self.Mem_Cached= line_vector[4];
        
    }
 
    pub fn convert_to_protobuf(&self)-> MemUsageProtobuf{
        MemUsageProtobuf{
            mem_total: self.Mem_Total,
            mem_free: self.Mem_Free,
            mem_available: self.Mem_Available,
            mem_buffer: self.Mem_Buffer,
            mem_cached: self.Mem_Cached
        }
    }
}

