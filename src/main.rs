//! RusticLab

pub use visadef::*;
pub use visafn::*;
pub use visatype::*;
/// static constants for attributes, error codes, etc.
mod visadef;
/// the extern-defined foreign function interface to VISA library.
mod visafn;
/// special VISA types that represent status, sessions, etc.
mod visatype;

fn main() {
    println!("Hello, world!");
}
