use crate::CpuUsageProtobuf; //Imported protobuf
                             //definitions from main
use crate::protobuf::CpuUsageRequest; //imports
use regex::Regex;
use std::{
    collections::HashMap, fs::File, io::prelude::*, io::BufReader, sync::mpsc::channel, thread,
    time,
}; //Std

#[derive(Debug)]
pub struct MultiCpuUsage {
    old_user_usage: HashMap<i32, u32>,
    old_nice_usage: HashMap<i32, u32>,
    old_system_usage: HashMap<i32, u32>,
    old_idle_usage: HashMap<i32, u32>,
    old_iowait_usage: HashMap<i32, u32>,
    old_irq_usage: HashMap<i32, u32>,
    old_softirq_usage: HashMap<i32, u32>,
    old_steal_usage: HashMap<i32, u32>,
    old_guest_usage: HashMap<i32, u32>,
    old_guest_nice_usage: HashMap<i32, u32>,
    user_usage: HashMap<i32, u32>,
    nice_usage: HashMap<i32, u32>,
    system_usage: HashMap<i32, u32>,
    idle_usage: HashMap<i32, u32>,
    iowait_usage: HashMap<i32, u32>,
    irq_usage: HashMap<i32, u32>,
    softirq_usage: HashMap<i32, u32>,
    steal_usage: HashMap<i32, u32>,
    guest_usage: HashMap<i32, u32>,
    guest_nice_usage: HashMap<i32, u32>,
}

//Struct creation relative to value generated from the command so be careful, otherwise you could lose on some data.
impl MultiCpuUsage {
    pub fn new() -> MultiCpuUsage {
        //Needs error checking for indexing
        MultiCpuUsage {
            old_user_usage: HashMap::new(),
            old_nice_usage: HashMap::new(),
            old_system_usage: HashMap::new(),
            old_idle_usage: HashMap::new(),
            old_iowait_usage: HashMap::new(),
            old_irq_usage: HashMap::new(),
            old_softirq_usage: HashMap::new(),
            old_steal_usage: HashMap::new(),
            old_guest_usage: HashMap::new(),
            old_guest_nice_usage: HashMap::new(),
            user_usage: HashMap::new(),
            nice_usage: HashMap::new(),
            system_usage: HashMap::new(),
            idle_usage: HashMap::new(),
            iowait_usage: HashMap::new(),
            irq_usage: HashMap::new(),
            softirq_usage: HashMap::new(),
            steal_usage: HashMap::new(),
            guest_usage: HashMap::new(),
            guest_nice_usage: HashMap::new(),
        }
    }
    fn update_values(&mut self, line_vector: Vec<&str>) {
        let re = Regex::new(r"cpu(\d{1,})").unwrap();
        let hash_map_key: i32;
        if re.is_match(line_vector[0].trim()) {
            let hash_map_key_temp: &str = &re.captures_iter(line_vector[0]).nth(0).unwrap()[1];
            hash_map_key = match hash_map_key_temp.trim().parse::<i32>() {
                Ok(n) => n + 1,
                _ => {
                    panic!("Something is going seriously wrong!!")
                }
            };
        } else {
            hash_map_key = 0;
        };
        self.old_user_usage.insert(
            hash_map_key,
            *self.user_usage.entry(hash_map_key).or_insert(0),
        );
        self.old_nice_usage.insert(
            hash_map_key,
            *self.nice_usage.entry(hash_map_key).or_insert(0),
        );
        self.old_system_usage.insert(
            hash_map_key,
            *self.system_usage.entry(hash_map_key).or_insert(0),
        );
        self.old_idle_usage.insert(
            hash_map_key,
            *self.idle_usage.entry(hash_map_key).or_insert(0),
        );
        self.old_iowait_usage.insert(
            hash_map_key,
            *self.iowait_usage.entry(hash_map_key).or_insert(0),
        );
        self.old_irq_usage.insert(
            hash_map_key,
            *self.irq_usage.entry(hash_map_key).or_insert(0),
        );
        self.old_softirq_usage.insert(
            hash_map_key,
            *self.softirq_usage.entry(hash_map_key).or_insert(0),
        );
        self.old_steal_usage.insert(
            hash_map_key,
            *self.steal_usage.entry(hash_map_key).or_insert(0),
        );
        self.old_guest_usage.insert(
            hash_map_key,
            *self.guest_usage.entry(hash_map_key).or_insert(0),
        );
        self.old_guest_nice_usage.insert(
            hash_map_key,
            *self.guest_nice_usage.entry(hash_map_key).or_insert(0),
        );

        self.user_usage
            .insert(hash_map_key, line_vector[1].trim().parse().unwrap());
        self.nice_usage
            .insert(hash_map_key, line_vector[2].trim().parse().unwrap());
        self.system_usage
            .insert(hash_map_key, line_vector[3].trim().parse().unwrap());
        self.idle_usage
            .insert(hash_map_key, line_vector[4].trim().parse().unwrap());
        self.iowait_usage
            .insert(hash_map_key, line_vector[5].trim().parse().unwrap());
        self.irq_usage
            .insert(hash_map_key, line_vector[6].trim().parse().unwrap());
        self.softirq_usage
            .insert(hash_map_key, line_vector[7].trim().parse().unwrap());
        self.steal_usage
            .insert(hash_map_key, line_vector[8].trim().parse().unwrap());
        self.guest_usage
            .insert(hash_map_key, line_vector[9].trim().parse().unwrap());
        self.guest_nice_usage
            .insert(hash_map_key, line_vector[10].trim().parse().unwrap());
    }
    //We are essentially implementing line vector generated from the command, onto the CpuUsage structure.

    fn sum_of_all_new_work(&self, hash_map_key: i32) -> u32 {
        self.user_usage.get(&hash_map_key).unwrap()
            + self.nice_usage.get(&hash_map_key).unwrap()
            + self.system_usage.get(&hash_map_key).unwrap()
            + self.iowait_usage.get(&hash_map_key).unwrap()
            + self.irq_usage.get(&hash_map_key).unwrap()
            + self.softirq_usage.get(&hash_map_key).unwrap()
            + self.steal_usage.get(&hash_map_key).unwrap()
            + self.guest_usage.get(&hash_map_key).unwrap()
            + self.guest_nice_usage.get(&hash_map_key).unwrap()
    }
    fn sum_of_all_old_work(&self, hash_map_key: i32) -> u32 {
        //Unwrap are safe only if caller function has a unwrap check on requested key value.

        self.old_user_usage.get(&hash_map_key).unwrap()
            + self.old_nice_usage.get(&hash_map_key).unwrap()
            + self.old_system_usage.get(&hash_map_key).unwrap()
            + self.old_iowait_usage.get(&hash_map_key).unwrap()
            + self.old_irq_usage.get(&hash_map_key).unwrap()
            + self.old_softirq_usage.get(&hash_map_key).unwrap()
            + self.old_steal_usage.get(&hash_map_key).unwrap()
            + self.old_guest_usage.get(&hash_map_key).unwrap()
            + self.old_guest_nice_usage.get(&hash_map_key).unwrap()
    }

    fn calculate_recent_usage(&self, cpu_id: i32) -> f32 {
        let record1_work = self.sum_of_all_new_work(cpu_id) as f32;
        let record2_work = self.sum_of_all_old_work(cpu_id) as f32;
        let record1_idle = *self.idle_usage.get(&cpu_id).unwrap() as f32;
        let record2_idle = *self.old_idle_usage.get(&cpu_id).unwrap() as f32;

        let cpu_usage = ((record1_work - record2_work)
            / ((record1_work + record1_idle) - (record2_idle + record2_work)))
            * 100.0;
        println!(
            "{} {} {} {}",
            cpu_usage, record2_work, record1_work, record1_idle
        );
        cpu_usage + 0.1
    }

    //We are using self to obtain the value from the CpuUsage structure as it is being implemented here.
    pub fn convert_to_protobuf(&self, req_payload: CpuUsageRequest) -> CpuUsageProtobuf {
        println!("{:?}", req_payload);
        let needed_cpu_id: i32 = match req_payload.needed_cpu_usage.parse() {
            Ok(n) => {
                if self.irq_usage.contains_key(&n) {
                    n
                } else {
                    print!("{}", n);
                    panic!("Invalid CPU ID")
                }
            }
            _ => {
                panic!("Junk CPU ID usage requested")
            }
        };
        CpuUsageProtobuf {
            cpu_id: String::from(needed_cpu_id.to_string()),
            cpu_usage: self.calculate_recent_usage(needed_cpu_id) as f32,
        }
    }
}

pub async fn main_cpu_stat_handler(statefull_cpu_usage: &mut MultiCpuUsage) {
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
        for i in lines.enumerate() {
            let main_cpu_info_vector: Vec<&str> = (i.1).split(' ').collect();
            let mut main_cpu_info_vector_sanitized: Vec<&str> = Vec::new();
            //Sanitizing vector
            for item in main_cpu_info_vector.iter() {
                if item.to_string() == "" {
                    continue;
                }
                main_cpu_info_vector_sanitized.push(item)
            }
            //Total cpu usage from boot:
            let re = Regex::new(r"cpu").unwrap();
            if re.is_match(main_cpu_info_vector_sanitized[0]) {
                statefull_cpu_usage.update_values(main_cpu_info_vector_sanitized);
            } else {
                break;
            }
        }
        thread::sleep(time::Duration::from_millis(100));
    }
}
