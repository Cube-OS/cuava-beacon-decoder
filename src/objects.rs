use chrono::TimeZone;

use crate::adcs::telemetry::Telemetry;
use crate::eps::*;
use crate::trxvu::{Telemetry as TrxvuTelemetry};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug)]
pub enum Packet {
    Beacon(Vec<Beacon>),
    Vector(Vec<u8>),
    Dummy(String),
}
impl TryFrom<Vec<u8>> for Packet {
    type Error = std::io::Error;

    fn try_from(vec: Vec<u8>) -> Result<Self, Self::Error> {
        match Beacon::deserialize(&vec) {
            Ok(beacon) => Ok(Packet::Beacon(beacon)),
            Err(_) => match String::from_utf8(vec.clone()) {
                Ok(s) => Ok(Packet::Dummy(s)),
                Err(_) => Ok(Packet::Vector(vec)),
            },
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum BeaconEnum {
    Eps(PIUHk),
    Adcs(Telemetry, Telemetry),
    Trxvu(TrxvuTelemetry, TrxvuTelemetry),
}
impl BeaconEnum {
    pub fn to_string(&self) -> String {
        match self {
            BeaconEnum::Eps(piuhk) => {
                format!("EPS: {}", serde_json::to_string_pretty(piuhk).unwrap())
            },
            BeaconEnum::Adcs(state, measurements) => {
                format!("ADCS State: {}\nADCS Measurements: {}", serde_json::to_string_pretty(state).unwrap(), serde_json::to_string_pretty(measurements).unwrap())
            },
            BeaconEnum::Trxvu(tx, rx) => {
                format!("TX: {}\nRX: {}", serde_json::to_string_pretty(tx).unwrap(), serde_json::to_string_pretty(rx).unwrap())
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Beacon {
    pub sat: String,
    pub time: i64,
    pub typ: BeaconEnum,
}
impl Beacon {
    pub fn to_string(&self) -> String {
        let mut s = format!("Sat: {:?}\nTime: {:?}\n", self.sat, chrono::Local.timestamp_opt(self.time,0).unwrap());
        s.push_str(&self.typ.to_string());
        s
    }
    pub fn deserialize(serialized: &[u8]) -> Result<Vec<Beacon>, std::io::Error> {
        // let (data, sat) = find_string_in_beacon(serialized.to_owned()).unwrap();        
        // let time = bincode::deserialize::<i32>(&data[1..6]).unwrap();
        // let typ = bincode::deserialize::<BeaconEnum>(&data[8..]).unwrap();        
        let beacons = find_strings_in_beacon(serialized.to_owned()).unwrap();
        let mut result = Vec::new();
        for beacon in beacons {
            let (sat, data) = beacon;
            println!("Data: {:?}", data);
            let (time, _, typ) = bincode::deserialize::<(i32, [u8; 2], BeaconEnum)>(&data[1..]).unwrap();    
            result.push(Beacon {
                sat: sat.to_string(),
                time: time.into(),
                typ,
            });
        }

        Ok(
            result
        )
    }
}
fn find_next_string_in_beacon(data: Vec<u8>) -> Option<usize> {
    let target_cuava = "CUAV".as_bytes();
    let target_ws = "WS-1".as_bytes();

    for i in 0..(data.len()-4) {
        if &data[i..i+4] == target_cuava {
            println!("Found CUAVA at {}", i);
           return Some(i);
        } else if &data[i..i+4] == target_ws {
            println!("Found WS-1 at {}", i);
            return Some(i);
        }
    }
    None
}
fn find_strings_in_beacon(data: Vec<u8>) -> Result<Vec<(Satellite, Vec<u8>)>, std::io::Error> {
    let target_cuava = "CUAV".as_bytes();
    let target_ws = "WS-1".as_bytes();
    let mut results = Vec::new();
    let mut i = 0;

    while i < data.len() - 4 {
        if &data[i..i+4] == target_cuava {            
            if &data[i+4..i+7] == "A-2".as_bytes() {
                match find_next_string_in_beacon(data[i+7..].to_vec()) {
                    Some(j) => {
                        results.push((Satellite::Cuava2, data[i+7..i+7+j-17].to_vec()));
                        i = i+7+j; // Move past the matched string
                    }
                    None => {
                        results.push((Satellite::Cuava2, data[i+7..].to_vec()));
                        i += 7; // Move past the matched string
                    }
                }
            } else if &data[i+4..i+7] == "A-E".as_bytes() {
                match find_next_string_in_beacon(data[i+8..].to_vec()) {
                    Some(j) => {
                        results.push((Satellite::CuavaEm, data[i+8..i+8+j-17].to_vec()));
                        i = i+8+j; // Move past the matched string
                    }
                    None => {
                        results.push((Satellite::CuavaEm, data[i+8..].to_vec()));
                        i += 8; // Move past the matched string
                    }
                }
            } else {
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "CUAVA not found in beacon"));
            }
        } else if &data[i..i+4] == target_ws {
            match find_next_string_in_beacon(data[i+4..].to_vec()) {
                Some(j) => {
                    results.push((Satellite::Ws1, data[i+4..i+4+j-17].to_vec()));
                    i = i+4+j; // Move past the matched string
                }
                None => {
                    results.push((Satellite::Ws1, data[i+4..].to_vec()));
                    i += 4; // Move past the matched string
                }
            }
        } else {
            i += 1; // Move to the next byte
        }
    }

    if results.is_empty() {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "String not found in beacon"));
    }

    Ok(results)
}

pub enum Satellite {
    Ws1,
    Cuava2,
    CuavaEm,
}
impl FromStr for Satellite {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "WS-1" => Ok(Satellite::Ws1),
            "CUAVA-2" => Ok(Satellite::Cuava2),
            "CUAVA-EM" => Ok(Satellite::CuavaEm),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid satellite name",
            )),
        }
    }
}
impl ToString for Satellite {
    fn to_string(&self) -> String {
        match self {
            Satellite::Ws1 => "WS-1".to_string(),
            Satellite::Cuava2 => "CUAVA-2".to_string(),
            Satellite::CuavaEm => "CUAVA-EM".to_string(),
        }
    }
}