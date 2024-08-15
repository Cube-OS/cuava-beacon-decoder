# CUAVA Beacon Decoder

This repository contains the application needed to parse the beacons from the University of Sydney's satellites CUAVA-2 and Waratah Seed-1. 

Additionally we include the gnu-radio flowgraph to receive the raw beacons.

## Application

The beacon decoder is written in Rust. To run the parser you'll need to:

- Clone this repo `git clone https://github.com/Cube-OS/cuava-beacon-decoder`
- [Install Rust](https://www.rust-lang.org/tools/install) if needed
- Change to the directory `cd cuava-beacon-decoder`

#### Run the parser

The parser can be used standalone by inputting a file with demodulated and decoded raw beacon data or together with the flowgraph, which requires the flowgraph to be started first.

- Run from the directory
```
cargo run
```
- Build and run from anywhere
```
cargo build --release
cp target/release/cuava-beacon-decoder PATH/TO/DESTINATION
cd PATH/TO/DESTINATION
./cuava-beacon-decoder
```

The following options are available:

```
./cuava-beacon-decoder -i INPUT-FILE -o OUTPUT-FILE -p PORT
```
The `-i INPUT-FILE` option enables the standalone use for parsing pre-recorded beacons.

The standard port is `3210`.

The JSON output is directed to a file `./beacon.txt` or any file specified with `-o OUTPUT-FILE`.

## Example

An example of a raw beacon is included in this repository and can be tested using
```
cargo run --release -- -i beacon.raw
```

## Transmission

Frequency: __400.65MHz__

Baudrate: __1k2__

Modulation: __BPSK-G3RUH__

The flowgraph has been built with `gnu-radio 3.9.6` and `gr-satellites 2.3.2` and has been tested with the __Ettus Research B200/B210__ SDRs.
