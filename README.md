# YClass

# Plugin API
Required functions:
* `fn yc_attach(process_id: u32) -> u32`
* `fn yc_read(address: usize, buffer: *mut u8, buffer_size: usize) -> u32`
* `fn yc_detach()`
