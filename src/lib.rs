//! TARS is a Rust library providing useful technical indicators such as Simple Moving
//! Average or Exponential Moving Average. This library is written in pure Rust and heavily
//! tested against the reference library in the field, TA-Lib.
//!
//! Implemented indicators :
//! - [x] Simple Moving Average
//! - [x] Exponential Moving Average
//! - [x] Parabolic SAR
//! - [x] RSI

pub mod overlap_studies;
pub mod momentum_indicators;
pub mod error;
pub mod helpers;
