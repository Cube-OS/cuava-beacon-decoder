//
// Copyright (C) 2022 CUAVA
//
// Licensed under the Apache License, Version 2.0 (the "License")
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
// 
// Contributed by: Patrick Oppel (patrick.oppel94@gmail.com)

use std::convert::{TryFrom,From};
use serde::*;
use std::fmt::{Debug};

///Radio function return values
#[derive(PartialEq, Serialize, Deserialize)]
pub enum Status {
    Ok = 0,
    RxEmpty,
    Error,
    ErrorConfig,
}

///Flags used to set transmitter's idle state
#[derive(Clone,Default,Debug,PartialEq,Serialize, Deserialize)]
pub enum IdleState {
    IdleOff,
    #[default]
    IdleOn,
}

///Telemetry request types
#[derive(Clone,Default,Debug,PartialEq,Serialize, Deserialize)]
pub enum TelemType {
    #[default]
    TxTelemAll,
    TxTelemLast,
    TxAx25,
    TxFreq,
    TxPllErrCount,    
    TxUptime,
    TxState,
    TxPaOverTemp,
    TxFirmware,
    TxLastReset,
    RxTelemAll,
    RxFreq,
    RxPllErrCount,
    RxUptime,
    RxFirmware,
    RxLastReset,
}

///TX/RX properties
#[derive(Clone,Serialize, Deserialize)]
pub struct TrxProp {
    pub addr: u8,
    pub max_size: u16,
    pub max_frames: u16,
}

///AX.25 call-sign structure
#[derive(Default,Clone, Debug, PartialEq,Serialize, Deserialize)]
pub struct AX25Callsign {
    pub ascii: String,
    pub ssid: u8,
}
impl AX25Callsign {
    pub fn new(ascii: String, ssid: u8) -> Self {
        AX25Callsign { ascii, ssid }
    }
}
impl TryFrom<Vec<u8>> for AX25Callsign {
    type Error = std::io::Error;

    fn try_from(v: Vec<u8>) -> Result<AX25Callsign, Self::Error> {
        if v.len() < 7 {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Not enough data"));
        }

        let ascii = std::str::from_utf8(&v[0..6])
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid ASCII data"))?
            .to_string();
        let ssid = v[6];

        Ok(AX25Callsign { ascii, ssid })
    }
}
impl From<AX25Callsign> for Vec<u8> {
    fn from(a: AX25Callsign) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend_from_slice(a.ascii.as_bytes());
        result.push(a.ssid);
        result
    }
}


#[derive(Debug,Default,Copy,Clone,Serialize,Deserialize)]
pub struct CommonTelemetry {
    pub voltage: u16,
    pub total_current: u16,
    pub tx_current: u16,
    pub rx_current: u16,
    pub pa_current: u16,
    pub pa_temp: u16,
    pub oscillator_temp: u16,
}
impl TryFrom<Vec<u8>> for CommonTelemetry {
    type Error = std::io::Error;

    fn try_from(v: Vec<u8>) -> Result<CommonTelemetry, Self::Error> {
        if v.len() < 14 {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Not enough data"));
        }

        Ok(CommonTelemetry {
            voltage: ((v[1] as u16)<<8) + (v[0] as u16),
            total_current: ((v[3] as u16)<<8) + (v[2] as u16),
            tx_current: ((v[5] as u16)<<8) + (v[4] as u16),
            rx_current: ((v[7] as u16)<<8) + (v[6] as u16),
            pa_current: ((v[9] as u16)<<8) + (v[8] as u16),
            pa_temp: ((v[11] as u16)<<8) + (v[10] as u16),
            oscillator_temp: ((v[13] as u16)<<8) + (v[12] as u16),
        })
    }
}

#[derive(Debug,Default,Copy,Clone,Serialize,Deserialize)]
pub struct TxTelemetry {
    pub rf_reflected: u16,
    pub rf_forward: u16,
    pub telemetry: CommonTelemetry,
}
impl TryFrom<Vec<u8>> for TxTelemetry {
    type Error = std::io::Error;

    fn try_from(v: Vec<u8>) -> Result<TxTelemetry, Self::Error> {
        if v.len() < 18 {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Not enough data"));
        }

        Ok(TxTelemetry {
            rf_reflected: (u16::from(v[1]) << 8) + u16::from(v[0]),
            rf_forward: (u16::from(v[3]) << 8) + u16::from(v[2]),
            telemetry: CommonTelemetry::try_from(v[4..18].to_vec())?,
        })
    }
}

#[derive(Debug,Default,Copy,Clone,Serialize,Deserialize)]
pub struct RxTelemetry {
    pub doppler_offset: u16,
    pub signal_strength: u16,
    pub telemetry: CommonTelemetry,
    pub packed_doppler: u16,
    pub packed_rssi: u16,
}
impl TryFrom<Vec<u8>> for RxTelemetry {
    type Error = std::io::Error;

    fn try_from(v: Vec<u8>) -> Result<RxTelemetry, Self::Error> {
        if v.len() < 22 {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Not enough data"));
        }

        Ok(RxTelemetry {
            doppler_offset: ((v[1] as u16)<<8) + (v[0] as u16),
            signal_strength: ((v[3] as u16)<<8) + (v[2] as u16),
            telemetry: CommonTelemetry::try_from(v[4..18].to_vec())?,
            packed_doppler: ((v[19] as u16)<<8) + (v[18] as u16),
            packed_rssi: ((v[21] as u16)<<8) + (v[20] as u16),
        })
    }
}

#[derive(Default,Debug,Copy,Clone,Serialize,Deserialize, )]
pub enum BitRate {
    B1200,
    B2400,
    B4800,
    #[default]
    B9600,
}
impl TryFrom<u8> for BitRate {
    type Error = std::io::Error;

    fn try_from(u: u8) -> std::io::Result<BitRate> {
        match (u >> 2) & 0x03 {
            0x00 => Ok(BitRate::B1200),
            0x01 => Ok(BitRate::B2400),
            0x02 => Ok(BitRate::B4800),
            0x03 => Ok(BitRate::B9600),
            _ => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid bit rate")),
        }
    }
}
impl From<BitRate> for u8 {
    fn from(val: BitRate) -> Self {
        match val {
            BitRate::B1200 => 0x01,
            BitRate::B2400 => 0x02,
            BitRate::B4800 => 0x04,
            BitRate::B9600 => 0x08,
        }
    }
}

#[derive(Debug,Copy,Clone,Serialize,Deserialize)]
pub struct TransmitterState {
    idle_on: bool,
    beacon: bool,
    pub bit_rate: BitRate,
}
impl TryFrom<Vec<u8>> for TransmitterState {
    type Error = std::io::Error;

    fn try_from(v: Vec<u8>) -> std::io::Result<TransmitterState> {
        match BitRate::try_from(v[0]) {
            Ok(b) => Ok(
                TransmitterState {
                idle_on: (v[0] & 0x01) == 1,
                beacon: (v[0] & 0x02) == 2,
                bit_rate: b,
            }),
            Err(e) => Err(e),
        }        
    }
}

#[derive(Copy,Clone,Serialize,Deserialize)]
pub enum PllPowerLevel {
    P4,
    P5,
}
impl From<PllPowerLevel> for Vec<u8> {
    fn from(p: PllPowerLevel) -> Vec<u8> {
        match p {
            PllPowerLevel::P4 => (0xFFCF_u16).to_le_bytes().to_vec(),
            PllPowerLevel::P5 => (0xEFCF_u16).to_le_bytes().to_vec(),
        }
    }
}


#[derive(Debug,Copy,Clone,Serialize,Deserialize)]
pub struct PllErrCount {
    pub lock: u16,
    pub freq: u16,
}
impl TryFrom<Vec<u8>> for PllErrCount {
    type Error = std::io::Error;

    fn try_from(v: Vec<u8>) -> Result<PllErrCount, Self::Error> {
        if v.len() < 4 {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Not enough data"));
        }

        Ok(PllErrCount {
            lock: ((v[1] as u16)<<8) + (v[0] as u16),
            freq: ((v[3] as u16)<<8) + (v[2] as u16),
        })
    }
}

#[derive(Default,Debug,Copy,Clone,Serialize,Deserialize)]
pub enum ResetCause {
    #[default]
    NoInterrupt,
    Brownout,
    RstNmi,
    Pmmswbor,
    WakeupFromLpm5,
    SecurityViolation,
    Svsl,
    Svsh,
    SvmlOvp,
    SvmhOvp,
    Pmmswpor,
    WatchdogTimer,
    WatchdogTimerPasswordViolation,
    FlashPasswordViolation,
    PeripheralAreaFetch,
    PmmPasswordViolation,
    Reserved,
}
impl From<u8> for ResetCause {
    fn from(v: u8) -> ResetCause {
        match v {
            0x00 => ResetCause::NoInterrupt,
            0x02 => ResetCause::Brownout,
            0x04 => ResetCause::RstNmi,
            0x06 => ResetCause::Pmmswbor,
            0x08 => ResetCause::WakeupFromLpm5,
            0x0A => ResetCause::SecurityViolation,
            0x0C => ResetCause::Svsl,
            0x0E => ResetCause::Svsh,
            0x10 => ResetCause::SvmlOvp,
            0x12 => ResetCause::SvmhOvp,
            0x14 => ResetCause::Pmmswpor,
            0x16 => ResetCause::WatchdogTimer,
            0x18 => ResetCause::WatchdogTimerPasswordViolation,
            0x1A => ResetCause::FlashPasswordViolation,
            0x1E => ResetCause::PeripheralAreaFetch,
            0x20 => ResetCause::PmmPasswordViolation,
            _ => ResetCause::Reserved,
        }
    }
}

#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct Ax25Telem {
    pub to: AX25Callsign,
    pub from: AX25Callsign,
}

///High-level Unifying Radio Telemetry Structure
#[derive(Debug,Clone,Serialize, Deserialize)]
pub enum Telemetry {
    Ax25(Ax25Telem),
    TxState(TransmitterState),
    Uptime(u32),
    Freq(u32),
    Firmware(String),
    ResetCause(ResetCause),
    PllErrCount(PllErrCount),
    TxTelemetry(TxTelemetry),
    RxTelemetry(RxTelemetry),   
    PaOverTempCount(bool),
}