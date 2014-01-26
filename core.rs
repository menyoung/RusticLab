#[crate_id = "RusticLab#0.1"];
#[crate_type = "lib"];

//! RusticLab's core Library
//!
//! provides traits and implementations for instruments, etc.
//! Tested with NI-VISA 5.4 for i686-apple-darwin Mac OS X 10.9.
//! 
//! Compile: `rustc --target i686-apple-darwin -L. core.rs`

#[feature(globs)];

extern mod visa;

use visa::*;

pub trait Instrument {
	fn setup(&self, chan: &str);
	fn source(&self, chan: &str, val: f64);
	fn measure(&self, chan: &str) -> f64;
}

pub trait Buffered : Instrument {
	fn trigger(&self);
	fn query(&self, chan: &str) -> ~[f64];
}

pub struct Keithley;

impl Instrument for Keithley {
	fn setup(&self, chan: &str) {
		println!("{}", chan);
	}
	fn source(&self, chan: &str, val: f64) {
		println!("{}: {}", chan, val);
	}
	fn measure(&self, chan: &str) -> f64 {
		println!("{}", chan);
		0.0
	}
}