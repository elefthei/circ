//! # CirC
//!
//! A compiler infrastructure for compiling programs to circuits

#![feature(box_patterns)]
#![feature(drain_filter)]
#![warn(missing_docs)]

#[macro_use]
pub mod ir;
pub mod circify;
pub mod front;
pub mod target;
pub mod util;
