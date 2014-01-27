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

pub use visa::*;
// use std::hashmap::{HashMap, HashSet};

pub trait Instrument {
	fn vi(&mut self) -> ViSession;
	fn init(&mut self);
	fn setup(&mut self);
	fn source(&mut self, chan: &[(&str, f64)]);
	fn measure(&mut self, chan: &[&str]) -> ~[(~str, f64)];
}

pub trait Buffered : Instrument {
	fn trigger(&mut self);
	fn readout(&mut self, chan: &str) -> ~[f64];
}

pub struct Keithley {
	addr: uint,
	instr: ViSession,
	steps: f64,
	delay: f64,
	compl: f64,
	range: f64
}

impl Instrument for Keithley {
	fn vi(&mut self) -> ViSession {
		self.instr
	}
	fn init(&mut self) {
		return;
	}
	fn setup(&mut self) {
		return;
	}
	fn source(&mut self, chan: &[(&str, f64)]) {
		println!("{}", chan.to_str());
	}
	fn measure(&mut self, chan: &[&str]) -> ~[(~str, f64)] {
		println!("{}", chan.to_str());
		let mut vals: ~[(~str, f64)] = ~[];
		for &ch in chan.iter() {
			println!("{}", ch);
			vals.push((ch.to_owned(), 0.003))
		}
		vals
	}
}

impl Buffered for Keithley {
	fn trigger(&mut self) {
		return;
	}
	fn readout(&mut self, chan: &str) -> ~[f64] {
		println!("{}", chan);
		~[0.0]
	}
}
