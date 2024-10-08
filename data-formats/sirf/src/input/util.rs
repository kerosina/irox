// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use irox_bits::{Bits, Error, MutBits};

pub fn write_gpstow<T: MutBits>(out: &mut T, val: f64) -> Result<(), Error> {
    let enc = (val * GPSTOW_SCALE).round() as u32;
    out.write_be_u32(enc)
}

pub fn read_gpstow<T: Bits>(out: &mut T) -> Result<f64, Error> {
    let read = out.read_be_u32()?;
    Ok(f64::from(read) / GPSTOW_SCALE)
}

pub const GPSTOW_SCALE: f64 = 100.0;
