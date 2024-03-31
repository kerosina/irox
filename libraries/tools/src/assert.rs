// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! Additional assert macros for tests
//!

#[macro_export]
macro_rules! assert_eq_eps {
    ($left:expr, $right:expr, $eps:expr) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                let delta = (*left_val - *right_val).abs();
                if !(delta <= $eps) {
                    panic!(
                        "Assertion failed, {} - {} = {} > {}",
                        &*left_val, &*right_val, delta, $eps
                    )
                }
            }
        }
    };
}

#[macro_export]
macro_rules! assert_eq_hex {
    ($left:expr, $right:expr) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if *left_val != *right_val {
                    panic!(
                        "Assertion failed, 0x{:0X} != 0x{:0X}",
                        &*left_val, &*right_val
                    )
                }
            }
        }
    };
}
