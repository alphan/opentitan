// Copyright lowRISC contributors.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

//! Capsule for measuring system call latency

use kernel::{AppId, Driver, ReturnCode};
use rv32i::csr::CSR;

pub const DRIVER_NUM: usize = 0xEFEFEFEF;

pub struct Latency {
}

impl Latency {
    pub fn new() -> Latency {
        Latency {
        }
    }
}

// Implement a command system call that reads `mcycle` to measure system call latency.
impl Driver for Latency {
    fn command(&self,
               _command_num: usize,
               _data: usize,
               _data2: usize,
               _app_id: AppId,
               ) -> ReturnCode {
        ReturnCode::SuccessWithValue{
            value: CSR.mcycle.get() as usize
        }
    }
}
