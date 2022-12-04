# YClass
A program that allows you to inspect and recreate data structures of other processes.

# Installation
To compile `YClass` you will need [Rust](https://www.rust-lang.org/tools/install).
```
git clone https://github.com/ItsEthra/yclass
cd yclass
cargo b --release
```

# Planned features
* [ ] - Writing values.
* [ ] - Save/Open project files.
* [ ] - Disassembly of function pointers.

# Plugin API
Required functions:
* `fn yc_attach(process_id: u32) -> u32`
* `fn yc_read(address: usize, buffer: *mut u8, buffer_size: usize) -> u32`
* `fn yc_detach()`
