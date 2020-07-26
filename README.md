## KFR(for capi) sys bindings
KFR is an open source C++ DSP framework that focuses on high performance. This repository contains the bindings for its capi only.

To build and use just do `cargo build`


## Notes
* Build on MacOs 10.15.5+
* Requires `cmake` and `clang++` present in the system


## Requirements:
* cmake

## Crates used and its purposes
* cmake => used for building the kfr library
* bindgen => Automatically generate Rust FFI bindings to C libraries. Header file used is `capi.h` in kfr library.

## License
It points to the license of kfr library