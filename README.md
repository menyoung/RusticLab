RusticLab
=========
<img src="http://upload.wikimedia.org/wikipedia/en/6/6a/Krusty_Krab.jpg"
 alt="Leiningen logo" title="The man himself" align="right" />

https://github.com/menyoung/RusticLab

All rights reserved, for now as long as repo is private.

-Menyoung.

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
