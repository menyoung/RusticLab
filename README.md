RusticLab
=========

https://github.com/menyoung/RusticLab

```
	let mut instr: ViSession = ViSession(25814);
	match visa::open(defaultRM, "GPIB0::12::INSTR", ViAccessMode(0), 0) {
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
	visa::set_attribute(ViObject(*instr), ViAttr(VI_ATTR_TMO_VALUE), ViAttrState(5000));
	// clear the instrument
	visa::clear(instr);
	
	// ask the device for identification
	match visa::write_str(instr, "*IDN?\n") {
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
	match visa::read_str(instr, MAX_CNT) {
		(status, msg, retCnt) => {
			println(msg);
			println(format!("Message was {} bytes long. Status = {}.", retCnt, *status as int));
			if (*status < VI_SUCCESS) {
				// error reading from instrument; exit.
				return;
			}
		}
	}

	visa::write_str(instr, "MEAS:VOLT:DC?");
	match visa::read_str(instr, MAX_CNT) {
		(status, msg, retCnt) => {
			println(msg);
			println(format!("Message was {} bytes long. Status = {}.", retCnt, *status as int));
			if (*status < VI_SUCCESS) {
				// error reading from instrument; exit.
				return;
			}
		}
	}
```

run with Agilent 34401A set to address 12 and connected to GPIB board outputs

```
Instrument at address 12 is 2033994752; Status = 0.
Upon write operation, RetCount = 6; Status = 0.
HEWLETT-PACKARD,34401A,0,3-1-1

Message was 31 bytes long. Status = 0.
-7.65800000E-06

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

