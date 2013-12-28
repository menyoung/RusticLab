RusticLab
=========

https://github.com/menyoung/RusticLab

All rights reserved, for now as long as repo is private.

-Menyoung.

Rust 0.8 instructions
---------------------
When you compile Rust, you have to include the architecture for which your VISA library was built, as a target triple.

```
./configure target-triples=(your system),(32-bit system compatible with NI VISA library)
make
sudo make install
```

On my system it was
	./configure --target-triples=x86_64-apple-darwin,i686-apple-darwin

compile to the right target arch!
	rustc --target i686-apple-darwin ex1.rc

