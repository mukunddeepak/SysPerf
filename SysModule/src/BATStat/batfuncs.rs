use crate::BatUsageProtobuf;
use std::{fs::File, io::prelude::*, io::BufReader, io::Read, thread, time};

#[derive(Debug)]
pub struct BatUsage {
    Charge_Full_Design: u32,
    Charge_Full: u32,
    Charge_Now: u32,
}

impl BatUsage {
    pub fn new() -> BatUsage {
        BatUsage {
            Charge_Full_Design: 0,
            Charge_Full: 0,
            Charge_Now: 0,
        }
    }

    fn update_values(&mut self, line_vector: Vec<u32>) {
        self.Charge_Full_Design = line_vector[0];
        self.Charge_Full = line_vector[1];
        self.Charge_Now = line_vector[2];
    }

    pub fn convert_to_protobuf(&self) -> BatUsageProtobuf {
        BatUsageProtobuf {
            charge_full_design: self.Charge_Full_Design,
            charge_full: self.Charge_Full,
            charge_now: self.Charge_Now,
        }
    }
}

pub async fn main_bat_stat_handler(statefull_bat_usage: &mut BatUsage) {
    println!("This is bat");
    loop {
        let statbatinfo_fd = match File::open("/sys/class/power_supply/BAT0/uevent") {
            Ok(x) => x,
            Err(_) => {
                // panic!("I use Arch BTW.");
                let statbatinfo_fd = match File::open("/sys/class/power_supply/BAT1/uevent") {
                    Ok(x) => x,
                    Err(_) => {
                        panic!("Ok, what the hell are you using?");
                    }
                };
                statbatinfo_fd
            }
        };

        let mut buff_reader = BufReader::new(&statbatinfo_fd);
        let mut batinfo = String::new();
        let _ = buff_reader.read_to_string(&mut batinfo);

        let mut lines: Vec<&str> = batinfo.split("\n").into_iter().collect();
        let mut temp_vec: Vec<u32> = Vec::new();
        // println!("{:?}", lines);
        let checker = lines[4].chars().position(|c| c == '=').unwrap() + 1;
        let power_model = lines[4].get(checker..).unwrap();

        // println!("{:?}", power_model);

        if (power_model == "Li-poly") {
            for i in 9..12 {
                let pos1 = lines[i].chars().position(|c| c == '=').unwrap() + 1;
                let finalvalue = lines[i].get(pos1..).unwrap();
                let line_val = finalvalue.trim().parse::<u32>().unwrap();
                temp_vec.push(line_val);
                // println!("{:?}", temp_vec);
            }
            statefull_bat_usage.update_values(temp_vec);
            thread::sleep(time::Duration::from_millis(100));
        } else {
            for i in 8..11 {
                let pos1 = lines[i].chars().position(|c| c == '=').unwrap() + 1;
                let finalvalue = lines[i].get(pos1..).unwrap();
                let line_val = finalvalue.trim().parse::<u32>().unwrap();
                temp_vec.push(line_val);
                // println!("{:?}", temp_vec);
            }
            statefull_bat_usage.update_values(temp_vec);
            thread::sleep(time::Duration::from_millis(100));
        }
    }
}
