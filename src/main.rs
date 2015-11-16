//! RusticLab main
#![allow(dead_code)]
#![allow(non_snake_case)]

extern crate libc;

/// special VISA types that represent status, sessions, etc.
mod visatype;
/// static constants for attributes, error codes, etc.
mod visadef;
/// the extern-defined foreign function interface to VISA library.
mod visafn;

fn main() {
    println!("Hello, world!");
}
