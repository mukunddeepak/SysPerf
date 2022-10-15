use crate::{CpuUsageProtobuf}; //Imported protobuf
                                                        //definitions from main
use std::{fs::File, io::prelude::*, io::BufReader, sync::mpsc::channel, thread, time}; //Std
                                                                                       //imports

#[derive(Debug)]
pub struct CpuUsage {
    cpu_id: String,
    old_user_usage: u32,
    old_nice_usage: u32,
    old_system_usage: u32,
    old_idle_usage: u32,
    old_iowait_usage: u32,
    old_irq_usage: u32,
    old_softirq_usage: u32,
    old_steal_usage: u32,
    old_guest_usage: u32,
    old_guest_nice_usage: u32,
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
    pub fn new() -> CpuUsage {
        //Needs error checking for indexing
        CpuUsage {
            cpu_id: String::from("0"),
            old_user_usage: 0,
            old_nice_usage: 0,
            old_system_usage: 0,
            old_idle_usage: 0,
            old_iowait_usage: 0,
            old_irq_usage: 0,
            old_softirq_usage: 0,
            old_steal_usage: 0,
            old_guest_usage: 0,
            old_guest_nice_usage: 0,
            user_usage: 0,
            nice_usage: 0,
            system_usage: 0,
            idle_usage: 0,
            iowait_usage: 0,
            irq_usage: 0,
            softirq_usage: 0,
            steal_usage: 0,
            guest_usage: 0,
            guest_nice_usage: 0,
        }
    }
    fn update_values(&mut self, line_vector: Vec<&str>) {
        self.old_user_usage = self.user_usage;
        self.old_nice_usage = self.nice_usage;
        self.old_system_usage = self.system_usage;
        self.old_idle_usage = self.idle_usage;
        self.old_iowait_usage = self.iowait_usage;
        self.old_irq_usage = self.irq_usage;
        self.old_softirq_usage = self.softirq_usage;
        self.old_steal_usage = self.steal_usage;
        self.old_guest_usage = self.guest_usage;
        self.old_guest_nice_usage = self.guest_nice_usage;

        self.user_usage = line_vector[1].trim().parse().unwrap();
        self.nice_usage = line_vector[2].trim().parse().unwrap();
        self.system_usage = line_vector[3].trim().parse().unwrap();
        self.idle_usage = line_vector[4].trim().parse().unwrap();
        self.iowait_usage = line_vector[5].trim().parse().unwrap();
        self.irq_usage = line_vector[6].trim().parse().unwrap();
        self.softirq_usage = line_vector[7].trim().parse().unwrap();
        self.steal_usage = line_vector[8].trim().parse().unwrap();
        self.guest_usage = line_vector[9].trim().parse().unwrap();
        self.guest_nice_usage = line_vector[10].trim().parse().unwrap();
    }
    //We are essentially implementing line vector generated from the command, onto the CpuUsage structure.

    fn sum_of_all_new_work(&self) -> u32 {
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
    fn sum_of_all_old_work(&self) -> u32 {
        self.old_user_usage
            + self.old_nice_usage
            + self.old_system_usage
            + self.old_iowait_usage
            + self.old_irq_usage
            + self.old_softirq_usage
            + self.old_steal_usage
            + self.old_guest_usage
            //register that allows the user to copy and pa
            + self.old_guest_nice_usage
    }

    fn calculate_recent_usage(&self) -> f32 {
        let record1_work = self.sum_of_all_new_work() as f32;
        let record2_work = self.sum_of_all_old_work() as f32;
        let record1_idle = self.idle_usage as f32;
        let record2_idle = self.old_idle_usage as f32;

        let cpu_usage = ((record1_work - record2_work)
            / ((record1_work + record1_idle) - (record2_idle + record2_work)))
            * 100.0;
        println!("{} {} {} {}",cpu_usage, record2_work, record1_work, record1_idle);
        cpu_usage
    }

    //We are using self to obtain the value from the CpuUsage structure as it is being implemented here.
    pub fn convert_to_protobuf(&self) -> CpuUsageProtobuf {
        println!("called , {}", self.calculate_recent_usage());
        CpuUsageProtobuf{
            cpu_id : String::from("0"),
            cpu_usage : self.calculate_recent_usage() as i32,
        }
    }
}

pub fn main_cpu_stat_handler(statefull_cpu_usage: &mut CpuUsage) {
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
        statefull_cpu_usage.update_values(main_cpu_info_vector_sanitized);
        thread::sleep(time::Duration::from_millis(100));
    }
}
