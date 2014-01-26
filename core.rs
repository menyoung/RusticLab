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
use std::hashmap::{HashMap, HashSet};

pub trait Instrument {
	fn vi(&self) -> ViSession;
	fn init(&self);
	fn setup(&self);
	fn source(&self, chan: HashMap<&str,f64>);
	fn measure(&self, chan: HashSet<&str>) -> HashMap<&str,f64>;
}

pub trait Buffered : Instrument {
	fn trigger(&self);
	fn readout(&self, chan: &str) -> ~[f64];
}

pub struct Keithley {
	vi: ViSession
}

impl Instrument for Keithley {
	fn vi(&self) -> ViSession {
		ViSession(0)
	}
	fn init(&self) {
		return;
	}
	fn setup(&self) {
		return;
	}
	fn source(&self, chan: HashMap<&str,f64>) {
		println!("{}", chan.to_str());
	}
	fn measure(&self, chan: HashSet<&str>) -> HashMap<&str,f64> {
		println!("{}", chan.to_str());
		HashMap::new()
	}
}

impl Buffered for Keithley {
	fn trigger(&self) {
		return;
	}
	fn readout(&self, chan: &str) -> ~[f64] {
		println!("{}", chan);
		~[0.0]
	}
}
