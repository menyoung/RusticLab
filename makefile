all: visa core ex

visa:
	rustc --target i686-apple-darwin visa.rs

core:
	rustc --target i686-apple-darwin -L. core.rs

ex: core visa
	rustc --target i686-apple-darwin -L. ex6.rs
