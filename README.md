# CUAVA Beacon Decoder

This repository contains the application needed to parse the beacons from the University of Sydney's satellites CUAVA-2 and Waratah Seed-1. 

Additionally we include the gnu-radio flowgraph to receive the raw beacons.

## Application

The beacon decoder is written in Rust. To run the parser you'll need to:

- Clone this repo `git clone https://github.com/Cube-OS/cuava-beacon-decoder`
- [Install Rust](https://www.rust-lang.org/tools/install) if needed
- Change to the directory `cd cuava-beacon-decoder`

#### Run the parser

The parser can be used standalone by inputting a file with demodulated and decoded raw beacon data.

```
cargo run -i /path/to/raw-file
```

Or together with the flowgraph. This requires the flowgraph to be started first.

```
cargo run
```

The standard port used is `3210` but can be altered with the option `-p PORT`.

The JSON output is directed to a file `./beacon.txt` or any file specified with `-o OUTPUT-FILE`.

## Transmission

Baudrate: __1k2__

Modulation: __BPSK-G3RUH__

The flowgraph has been built with `gnu-radio 3.9.6` and `gr-satellites 2.3.2` and has been tested with the __Ettus Research B200/B210__ SDRs.