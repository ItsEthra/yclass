//! This module parses addresses.
//! All numbers are parsed as hexidecimals.
//! Expected Syntax:
//! * `[0xAABB]` loads 8 bytes at address `0xAABB`.
//! * `<module.exe>` loads address of the `module.exe`.
//! Allowed operations are would be: `+`, `-`, `*`.

// TODO(ItsEthra): Parse address (with `nom` crate maybe?) to allow special syntax like adding, dereferencing
// pointers and getting modules' addresses.
pub fn parse_address(addr: &str) -> Option<usize> {
    addr.strip_prefix("0x").unwrap_or(addr).parse().ok()
}
