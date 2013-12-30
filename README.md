RusticLab
=========

https://github.com/menyoung/RusticLab

Here's ex.3:

```rust
#[feature(globs)];

extern mod visa;

use visa::*;

fn main() {
	println(format!("Hello. VISA Version is {}.{}.{}.", VI_VERSION_MAJOR(VI_SPEC_VERSION) as int, VI_VERSION_MINOR(VI_SPEC_VERSION) as int, VI_VERSION_SUBMINOR(VI_SPEC_VERSION) as int));
	
	// initialize the system.
	let mut defaultRM: ViSession = ViSession(25813);
	match vi_open_default_RM() {
		(status, resource_manager) => {
			*defaultRM = *resource_manager;
			println(format!("Default Resource Manager is {}; Status = {}.", *defaultRM as int, *status as int));
			if (*status < VI_SUCCESS) {
				// error initializing; exit.
				return;
			}
		}
	}
	
	// open communication with gpib device at primary address 12.
	// no error checking!
	let mut instr: ViSession = ViSession(25814);
	match vi_open(defaultRM, "GPIB0::12::INSTR", ViAccessMode(0), 0) {
		(status, vi) => {
			*instr = *vi;
			println(format!("Instrument at address 12 is {}; Status = {}.", *instr as int, *status as int));
			if (*status < VI_SUCCESS) {
				// error opening instrument; exit.
				return;
			}
		}
	}
	
	// set the timeout for messages
	vi_set_attribute(ViObject(*instr), ViAttr(VI_ATTR_TMO_VALUE), ViAttrState(5000));
	// clear the instrument
	vi_clear(instr);

	// ask the device for identification
	match vi_write_str(instr, "*IDN?\n") {
		(status, retCnt) => {
			println(format!("Upon write operation, RetCount = {}; Status = {}.", retCnt, *status as int));
			if (*status < VI_SUCCESS) {
				// error writing to instrument; exit.
				return;
			}
		}
	}
	
	// read the response
	let MAX_CNT = 200;
	match vi_read_str(instr, MAX_CNT) {
		(status, msg, retCnt) => {
			println(msg);
			println(format!("Message was {} bytes long. Status = {}.", retCnt, *status as int));
			if (*status < VI_SUCCESS) {
				// error reading from instrument; exit.
				return;
			}
		}
	}

	vi_write_str(instr, "MEAS:VOLT:DC?");
	match vi_read_str(instr, MAX_CNT) {
		(status, msg, retCnt) => {
			println(msg);
			println(format!("Message was {} bytes long. Status = {}.", retCnt, *status as int));
			if (*status < VI_SUCCESS) {
				// error reading from instrument; exit.
				return;
			}
		}
	}

	vi_close(instr);
	vi_close(defaultRM);
}
```

Compile:
```bash
rustc --target i686-apple-darwin visa.rs
rustc --target i686-apple-darwin ex3.rs -L.
```

running `./ex3` on my Mac with the NI-VISA 5.4 framework installed, with Agilent 34401A set to address 12 and connected to GPIB board outputs

```
Hello. VISA Version is 5.1.0.
Default Resource Manager is 2054567984; Status = 0.
Instrument at address 12 is 2067133952; Status = 0.
Upon write operation, RetCount = 6; Status = 0.
HEWLETT-PACKARD,34401A,0,3-1-1

Message was 31 bytes long. Status = 0.
+1.60750000E-05

Message was 16 bytes long. Status = 0.
```

Rust 0.9pre instructions
---------------------
When you compile Rust, you have to include the architecture for which your VISA library was built, as a target triple.

```
./configure --target=(your system),(32-bit system compatible with NI VISA library)
make
sudo make install
```

On my system it was
```
./configure --target=x86_64-apple-darwin,i686-apple-darwin
```

compile to the right target arch!
```
rustc --target i686-apple-darwin ex1.rs
```

VI types
--------

|VI type	|Rust type	|
|-----------|-----------|
|`ViUInt64`	|`u64`		|
|`ViInt64`	|`i64`		|
|`ViUInt32`	|`u32`		|
|`ViInt32`	|`i32`		|
|`ViUInt16`	|`u16`		|
|`ViInt16`	|`i16`		|
|`ViUInt8`	|`u8`		|
|`ViInt8`	|`i8`		|
|`ViChar`	|`c_char`	|
|`ViByte`	|`u8`		|
|`ViAddr`	|`~c_void`	|
|`ViReal32`	|`f32`		|
|`ViReal64`	|`f64`		|
|`ViBuf`	|`u8`		|
|`ViString`	|`~c_char`	|
|`ViRsrc`	|`~c_char`	|
|`ViBoolean`|`u16`		|
|`ViStatus`	|`i32`		|
|`ViVersion`|`u32`		|
|`ViObject`	|`u32`		|
|`ViSession`|`u32`		|
|`ViAttr`	|`u32`		|
|`ViEvent`	|`u32`		|
|`ViFindList`	|`u32`	|
|`ViBusAddress`	|`u32`	|
|`ViBusSize`	|`u32`	|
|`ViAttrState`	|`u32`	|
|`ViBusAddress64`|`u64`	|
|`ViEventType`	|`u32`	|
|`ViKeyId`	|`~c_char`_	|
|`ViJobId`	|`u32`		|
|`ViAccessMode`	|`u32`	|
|`ViEventFilter`|`u32`	|
|`ViVAList`		|????	|

I don't understand the function typedef for event handling yet.
```
typedef ViStatus (* ViHndlr) (ViSession vi, ViEventType eventType, ViEvent event, ViAddr userHandle);
```

Currently implemented only as 32bit system, and without function calling conventions directives.

All rights reserved, for now as long as repo is private.

-Menyoung.

