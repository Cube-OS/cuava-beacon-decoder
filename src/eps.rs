//
// Copyright (C) 2022 CUAVA, The University of Sydney
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

//! CubeOS API for interacting with [ISIS ICEPSv2]
// Reference documentation: ISIS Electrical Power System 2 –Software ICD – IVID 7

// The API is contributed by Xueliang Bai <x.bai@sydney.edu.au>
// and Oscar Wilfred Thomas Ansted <oans4023@uni.sydney.edu.au> on behalf of the
// ARC Training Centre for CubeSats, UAVs & Their Applications (CUAVA) team (www.cuava.com.au)
// at the University of Sydney

// Input enumerations
// System Type Identifier (STID)

use serde::*;

// The voltage V - current I - power P datatype (VIPD) data.
// Used in blocks across the HK telemetry.
#[derive(Clone, Debug, Default, Serialize, Deserialize, Hash)]
pub struct VIPData {
    pub volt: i16,
    pub curr: i16,
    pub pwr: i16,
}

impl From<Vec<u8>> for VIPData {
    fn from(v: Vec<u8>) -> VIPData {
        VIPData {
            volt: <i16>::from_le_bytes([v[0], v[1]]),
            curr: <i16>::from_le_bytes([v[2], v[3]]),
            pwr: 10 * (<i16>::from_le_bytes([v[4], v[5]])),
        }
    }
}

//CCSD, Short for conditioning channel datatype (CCD), withou VIP data
#[derive(Clone, Debug, Default, Serialize, Deserialize, Hash)]
pub struct CondChnShortData {
    volt_in_mppt: i16,
    curr_in_mppt: i16,
    volt_out_mppt: i16,
    curr_out_mppt: i16,
}

impl From<Vec<u8>> for CondChnShortData {
    fn from(v: Vec<u8>) -> CondChnShortData {
        CondChnShortData {
            volt_in_mppt: <i16>::from_le_bytes([v[0], v[1]]),
            curr_in_mppt: <i16>::from_le_bytes([v[2], v[3]]),
            volt_out_mppt: <i16>::from_le_bytes([v[4], v[5]]),
            curr_out_mppt: <i16>::from_le_bytes([v[6], v[7]]),
        }
    }
}

// PIU Housekeeping Engineering/Average Data (0xA2 and 0xA4)
#[derive(Clone, Debug, Default, Serialize, Deserialize, Hash)]
pub struct PIUHk {
    // One reseved byte. Starting from the 6th byte
    // Voltage of internal board supply.
    pub volt_brdsup: i16,
    // Measured temperature of the MCU
    pub temp: i16,
    // Input V, I and P input of the distribution part of the unit in raw form.
    pub vip_dist_input: VIPData,
    // Input V, I and P input of the battery part of the unit
    pub vip_batt_input: VIPData,
    // Bitflag field indicating channel-on status for output 0 through 15.
    pub stat_ch_on: u16,
    // Bitflag field indicating overcurrent latch-off fault for output 0 through 15.
    pub stat_ch_ocf: u16,
    // Bitflag field indicating BP board status.
    pub batt_stat: u16,
    // 2 and 4 cell battery pack
    pub batt_temp2: i16,
    // 2 cell battery pack not used, temp for 4 cell battery pack:
    pub batt_temp3: i16,
    // Voltage level for domain 0 - 2
    pub volt_vd0: i16,
    pub volt_vd1: i16,
    pub volt_vd2: i16,
    // VIPData output for channel 0 - 16
    // VD0_0, 3.3V
    pub vip_cnt_ch00: VIPData,
    // VD1_0, 5V
    pub vip_cnt_ch01: VIPData,
    // VD1_1, 5V
    pub vip_cnt_ch02: VIPData,
    // VD1_2, 5V
    pub vip_cnt_ch03: VIPData,
    // VD1_3, 3.3V
    pub vip_cnt_ch04: VIPData,
    // VD2_0, 3.3V
    pub vip_cnt_ch05: VIPData,
    // VD2_1, 3.3V
    pub vip_cnt_ch06: VIPData,
    // VD2_2, 3.3V
    pub vip_cnt_ch07: VIPData,
    // VD2_3, 3.3V
    pub vip_cnt_ch08: VIPData,
    // Data on conditioning chain
    pub ccd1: CondChnShortData,
    pub ccd2: CondChnShortData,
    pub ccd3: CondChnShortData,
    // VD0_1, 3.3V
    pub vip_cnt_ch09: VIPData,
    // VD0_2, 3.3V
    pub vip_cnt_ch10: VIPData,
    // VD0_3, 3.3V
    pub vip_cnt_ch11: VIPData,
    // VD3_0, 5.4V (customized)
    pub vip_cnt_ch12: VIPData,
    // VD3_1, 5.4V (customized)
    pub vip_cnt_ch13: VIPData,
    // VD4_0, 12V (customized)
    pub vip_cnt_ch14: VIPData,
    // VD4_1, 12V (customized)
    pub vip_cnt_ch15: VIPData,
    // Data on conditioning chain
    pub ccd4: CondChnShortData,
    pub ccd5: CondChnShortData,
    // Bitflag field indicating channel-on status for the extended output bus channels
    pub stat_ch_ext_on: u16,
    // Bitflag field indicating overcurrent latch-off fault status for the extended output bus channels
    pub stat_ch_ext_ocf: u16,
    // VD5_0, 28.2V (default)
    pub vip_cnt_ch16: VIPData,
    // Stop at 184 byte for the ICEPSv2
}
impl From<Vec<u8>> for PIUHk {
    fn from(v: Vec<u8>) -> PIUHk {
        PIUHk {
            volt_brdsup: <i16>::from_le_bytes([v[6], v[7]]),
            temp: <i16>::from_le_bytes([v[8], v[9]]),
            vip_dist_input: VIPData::from(v[10..16].to_vec()),
            vip_batt_input: VIPData::from(v[16..22].to_vec()),
            stat_ch_on: <u16>::from_le_bytes([v[22], v[23]]),
            stat_ch_ocf: <u16>::from_le_bytes([v[24], v[25]]),
            batt_stat: <u16>::from_le_bytes([v[26], v[27]]),
            batt_temp2: <i16>::from_le_bytes([v[28], v[29]]),
            batt_temp3: <i16>::from_le_bytes([v[30], v[31]]),
            volt_vd0: <i16>::from_le_bytes([v[32], v[33]]),
            volt_vd1: <i16>::from_le_bytes([v[34], v[35]]),
            volt_vd2: <i16>::from_le_bytes([v[36], v[37]]),
            vip_cnt_ch00: VIPData::from(v[38..44].to_vec()),
            vip_cnt_ch01: VIPData::from(v[44..50].to_vec()),
            vip_cnt_ch02: VIPData::from(v[50..56].to_vec()),
            vip_cnt_ch03: VIPData::from(v[56..62].to_vec()),
            vip_cnt_ch04: VIPData::from(v[62..68].to_vec()),
            vip_cnt_ch05: VIPData::from(v[68..74].to_vec()),
            vip_cnt_ch06: VIPData::from(v[74..80].to_vec()),
            vip_cnt_ch07: VIPData::from(v[80..86].to_vec()),
            vip_cnt_ch08: VIPData::from(v[86..92].to_vec()),
            ccd1: CondChnShortData::from(v[92..100].to_vec()),
            ccd2: CondChnShortData::from(v[100..108].to_vec()),
            ccd3: CondChnShortData::from(v[108..116].to_vec()),
            vip_cnt_ch09: VIPData::from(v[116..122].to_vec()),
            vip_cnt_ch10: VIPData::from(v[122..128].to_vec()),
            vip_cnt_ch11: VIPData::from(v[128..134].to_vec()),
            vip_cnt_ch12: VIPData::from(v[134..140].to_vec()),
            vip_cnt_ch13: VIPData::from(v[140..146].to_vec()),
            vip_cnt_ch14: VIPData::from(v[146..152].to_vec()),
            vip_cnt_ch15: VIPData::from(v[152..158].to_vec()),
            ccd4: CondChnShortData::from(v[158..166].to_vec()),
            ccd5: CondChnShortData::from(v[166..174].to_vec()),
            stat_ch_ext_on: <u16>::from_le_bytes([v[174], v[175]]),
            stat_ch_ext_ocf: <u16>::from_le_bytes([v[176], v[177]]),
            vip_cnt_ch16: VIPData::from(v[178..184].to_vec()),
        }
    }
}
