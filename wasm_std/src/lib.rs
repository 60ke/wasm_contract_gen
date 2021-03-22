#![cfg_attr(not(feature = "std"), no_std)]
// #![no_std]
// https://docs.rust-embedded.org/embedonomicon/smallest-no-std.html
// https://doc.rust-lang.org/1.7.0/book/using-rust-without-the-standard-library.html
#![feature(lang_items)]
#![feature(link_args)]
#![feature(core_intrinsics)]
#![feature(panic_info_message)]

// #![warn(missing_docs)]
#[allow(unused_attributes)]

pub mod types;
pub mod logs;
mod panic;
pub use alloc::boxed::Box;
pub use alloc::string::String;
pub use alloc::str;
pub use alloc::vec::Vec;

extern crate alloc;
#[cfg(feature = "std")]
extern crate core;

// #[no_mangle]
#[cfg(not(feature = "std"))]
pub use panic::panic_fmt;

// #[no_mangle]
#[cfg(not(feature = "std"))]
pub use panic::oom;




use byteorder::{LittleEndian, ByteOrder};
/// Read u32 using native endianness
pub fn read_u32(buf: &[u8]) -> u32 {
    LittleEndian::read_u32(buf)
}

/// Write u32 using native endianness
pub fn write_u32(buf: &mut [u8], val: u32) {
    LittleEndian::write_u32(buf, val)
}

/// Write ptr using native endianness
pub fn write_ptr(buf: &mut [u8], ptr: *mut u8) {
    // 判断架构位数
    assert!(cfg!(target_pointer_width = "64"));
    write_u32(buf, ptr as usize as u32);
}

/// Read u64 using native endianness
pub fn read_u64(buf: &[u8]) -> u64 {
    LittleEndian::read_u64(buf)
}

/// Write u64 using native endianness
pub fn write_u64(buf: &mut [u8], val: u64) {
    LittleEndian::write_u64(buf, val)
}





