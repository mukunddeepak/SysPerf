use crate::ChannelType;
use std::{fs::File, io::prelude::*, io::BufReader, sync::mpsc::channel, thread, time};

#[derive(Debug)]
pub struct CpuUsage {
    cpu_id: String,
    user_usage: u32,
    nice_usage: u32,
    system_usage: u32,
    idle_usage: u32,
    iowait_usage: u32,
    irq_usage: u32,
    softirq_usage: u32,
    steal_usage: u32,
    guest_usage: u32,
    guest_nice_usage: u32,
}
//Struct creation relative to value generated from the command so be careful, otherwise you could lose on some data.

impl CpuUsage {
    fn new(line_vector: Vec<&str>) -> CpuUsage {
        //Needs error checking for indexing
        CpuUsage {
            cpu_id: line_vector[0].to_string(),
            user_usage: line_vector[1].trim().parse().unwrap(),
            nice_usage: line_vector[2].trim().parse().unwrap(),
            system_usage: line_vector[3].trim().parse().unwrap(),
            idle_usage: line_vector[4].trim().parse().unwrap(),
            iowait_usage: line_vector[5].trim().parse().unwrap(),
            irq_usage: line_vector[6].trim().parse().unwrap(),
            softirq_usage: line_vector[7].trim().parse().unwrap(),
            steal_usage: line_vector[8].trim().parse().unwrap(),
            guest_usage: line_vector[9].trim().parse().unwrap(),
            guest_nice_usage: line_vector[10].trim().parse().unwrap(),
        }
    }
    //We are essentially implementing line vector generated from the command, onto the CpuUsage structure.

    fn sum_of_all_work(&self) -> u32 {
        self.user_usage
            + self.nice_usage
            + self.system_usage
            + self.iowait_usage
            + self.irq_usage
            + self.softirq_usage
            + self.steal_usage
            + self.guest_usage
            + self.guest_nice_usage
    }
    //We are using self to obtain the value from the CpuUsage structure as it is being implemented here.
}

fn calculate_recent_usage(timed_storage_buffer: &Vec<CpuUsage>) -> f32 {
    //https://www.linuxhowtos.org/System/procstat.htm , /proc/stat is total usage from boot
    let record1 = match timed_storage_buffer.last() {
        Some(x) => x,
        None => {
            println!("Please make sure you are using a lunux system!");
            panic!(
                "Error : seems like your buffer is empty! Error point[calculate_recent_usagege]"
            );
        }
    };
    let record2 = match timed_storage_buffer.get(timed_storage_buffer.len() - 2) {
        Some(x) => x,
        None => {
            //Not an error
            println!("Initiliziing");
            return 0.0;
        }
    };
    let record1_work = record1.sum_of_all_work() as f32;
    let record2_work = record2.sum_of_all_work() as f32;
    let record1_idle = record1.idle_usage as f32;
    let record2_idle = record2.idle_usage as f32;

    let cpu_usage = ((record1_work - record2_work)
        / ((record1_work + record1_idle) - (record2_idle + record2_work)))
        * 100.0;
    cpu_usage
}

pub async fn main_cpu_stat_handler(transmitter: std::sync::mpsc::Sender<ChannelType>) {
    loop {
        //Reading entire file from system
        let procstat_fd = match File::open("/proc/stat") {
            Ok(x) => x,
            Err(_) => {
                panic!("Make sure you are using a linux system! Error point[reading stat file]");
            }
        };
        let mut buff_reader = BufReader::new(&procstat_fd);
        let mut current_cpu_stat = String::new();
        let _ = buff_reader.read_to_string(&mut current_cpu_stat);
        // processing buffer to details
        let mut lines = current_cpu_stat.split("\n");
        let main_cpu_info_vector: Vec<&str> = lines.nth(0).unwrap().split(" ").collect();
        let mut main_cpu_info_vector_sanitized: Vec<&str> = Vec::new();
        //Sanitizing vector
        for item in main_cpu_info_vector.iter() {
            if item.to_string() == "" {
                continue;
            }
            main_cpu_info_vector_sanitized.push(item)
        }
        //Total cpu usage from boot:
        let current_usage = CpuUsage::new(main_cpu_info_vector_sanitized);
        transmitter
            .send(ChannelType::CpuData(current_usage))
            .unwrap();

        thread::sleep(time::Duration::from_millis(100));
    }
}
