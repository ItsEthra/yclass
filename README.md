# YClass
A program that allows you to inspect and recreate data structures of other processes.
![](./assets/showcase1.png)

# Installation
To compile `YClass` you will need [Rust](https://www.rust-lang.org/tools/install).
```
git clone https://github.com/ItsEthra/yclass
cd yclass
cargo r --release
```

# Features
* Preserving fields' offsets when other fields are replaced.
* Different variable types:
    * `I8`, `I16`, `I32`, `I64`
    * `U8`, `U16`, `U32`, `U64`
    * `F32`, `F64`
    * `Pointer`
    * `Bool`
* Generating Rust/C++ code out of classes.
* Saving/Opening project files.
* Plugin API to customize reading behavior.
* Preview of the memory pointer is pointing to.

# Hotkeys
* `Ctrl-C` - Copies selected field's address (in hex) to the clipboard.
* `Ctrl-Shift-C` - Copies selected field's value as 8 byte (in hex) to the clipboard.
* `Alt-A` - Open a window to select a process to attach.
* `Alt-Ctrl-A` - Attach to the most recent process.
* `Alt-D` - Detach from the process.

# Planned features
* [x] - ~~Writing values.~~
* [x] - ~~Save/Open project files.~~
* [x] - ~~Pointer preview on hover with unknown fields.~~
* [ ] - Disassembly of function pointers.

# Plugin API
You can write a plugin to change the way `YClass` reads memory.
To do that you will need a shared library(`.dll` or `.so`) that exports following functions
specified below. `u32` return value should be treated as status code. If it's `0` then no error is displayed.
Otherwise return value is displayed in the notification.
Required functions:
* `fn yc_attach(process_id: u32) -> u32` - Called when attaching to a process.
* `fn yc_read(address: usize, buffer: *mut u8, buffer_size: usize) -> u32` - Called(very frequently) when reading memory.
    * `address` is in attached process address space.
    * `buffer` is in current process address space.
* `fn yc_write(address: usize, buffer: *const u8, buffer_size: usize) -> u32` - Called(rarely) when writing memory.
    * `address` is in attached process address space.
    * `buffer` is in current process address space.
* `fn yc_can_read(address: usize) -> bool` - Called(mildly frequently) to check if address is "readable", i.e. a pointer.
    * `address` is in attached process address space.
* `fn yc_detach()` - Called when detaching from a process.
### After its done, put your library at `./plugin.ycpl` or specify the path under `plugin_path` key in your config.
Config path:
* Windows - `C:\Users\%USER%\AppData\Roaming\yclass\config.toml`
* Unix - `~/.config/yclass/config.toml`($XDG_CONFIG_HOME)
