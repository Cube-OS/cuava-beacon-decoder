mod objects;
mod trxvu;
mod eps;
mod adcs;

use objects::*;

use chrono::Local;
use clap::{App,Arg};

use std::fs::{OpenOptions, File};
use std::io::{BufReader, Read, Write};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let matches = App::new("cuava-beacon-decoder")
        .version("0.1.0")
        .author("Patrick Oppel")
        .about("Beacon decoder for CUAVA-2 and WS-1 satellites")
        .arg(Arg::with_name("input")
            .short('i')
            .long("input")
            .value_name("FILE")
            .help("Sets the input file to decode the beacon from")
            .takes_value(true)
        )
        .arg(Arg::with_name("output")
            .short('o')
            .long("output")
            .value_name("FILE")
            .help("Sets the output file to write the decoded beacon to")
            .takes_value(true)
        )
        .arg(Arg::with_name("port")
            .short('p')
            .long("port")
            .value_name("PORT")
            .help("Sets the port to connect to")
            .takes_value(true)
        )
        .get_matches();
    
    let input: Option<String> = matches.value_of("input").map(|s| s.to_string());
    let output = matches.value_of("output").unwrap_or("beacon.txt").to_string();
    let port = matches.value_of("port").unwrap_or("3210");

    let mut output_file = match OpenOptions::new()
        .append(true)
        .create(true)
        .write(true)
        .read(true)
        .open(output)
    {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open file: {}", e);
            return Err(e);
        }
    };

    match input {
        Some(file) => {
            let file = File::open(file).unwrap();
            let mut buffer = Vec::new();
            let mut file = BufReader::new(file);
            file.read_to_end(&mut buffer).unwrap();
            let beacons = Beacon::deserialize(&buffer).unwrap();
            output_file.write_all(handle_beacon(beacons)?.as_bytes())     
        }
        None => {
            let address = format!("127.0.0.1:{}", port);   
            let stream = TcpStream::connect(address.clone()).unwrap();
            let stream_clone = stream.try_clone().unwrap();

            read_loop(stream_clone, address.clone(), output_file)
        }
    }
}

fn read_loop(mut stream_clone: TcpStream, address: String, mut output: File) -> Result<(), std::io::Error> {
    let mut buffer = [0; 335];
    loop {
        match stream_clone.read(&mut buffer) {
            Ok(0) => {
                // Add a delay before retrying the connection
                std::thread::sleep(std::time::Duration::from_secs(1));
                stream_clone = TcpStream::connect(address.clone()).unwrap();
            }
            Ok(size) => {            
                match handle_message(&buffer[..size]) {
                    Ok(Packet::Beacon(data)) => match handle_beacon(data) {
                        Ok(s) => write(&mut output, s.as_bytes())?,
                        Err(e) => {
                            eprintln!("Error: {}", e);
                        }
                    },
                    Ok(_p) => handle_other(&buffer[..size]),
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to receive data: {}", e);
            }
        }
    }
}

fn handle_message(buffer: &[u8]) -> Result<Packet, std::io::Error> {
    let mut raw = match OpenOptions::new()
        .append(true)
        .create(true)
        .write(true)
        .read(true)
        .open("beacon_raw.txt")
    {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open file: {}", e);
            return Err(e);
        }
    };

    let raw_beacon = buffer.to_vec();
    write(&mut raw, &raw_beacon).unwrap();
    drop(raw);

    Packet::try_from(buffer.to_vec())
}

fn handle_beacon(beacons: Vec<Beacon>) -> Result<String, std::io::Error> {
    let mut buffer = Vec::new();
    for beacon in beacons {
        buffer.push(beacon.to_string());
    }
    buffer.push("".to_string());
    Ok(buffer.join("\n"))
}

fn write(file: &mut File, data: &[u8]) -> Result<(), std::io::Error> {
    match file.write_all(data) {
        Ok(_) => {
            Ok(())
        },
        Err(e) => {
            eprintln!("Failed to write to file: {}", e);
            Err(e)
        }
    }
}

fn handle_other(buffer: &[u8]) {
    let mut file = match OpenOptions::new()
        .append(true)
        .create(true)
        .open("other.txt")
    {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open file: {}", e);
            return;
        }
    };

    let current_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let message_with_time = format!("[{}] {}\n", current_time, String::from_utf8_lossy(buffer));

    if let Err(e) = file.write_all(message_with_time.as_bytes()) {
        eprintln!("Failed to write to file: {}", e);
    }
}
