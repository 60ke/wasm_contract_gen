// https://doc.rust-lang.org/1.7.0/book/no-stdlib.html
#![allow(warnings)]
#![cfg(not(feature = "std"))]
pub use alloc::vec::Vec;
use byteorder::{LittleEndian, ByteOrder};



extern "C" {
    fn panic(payload_ptr: *const u8, payload_len: u32) -> !;
}

#[cfg(not(feature = "panic_with_msg"))]
#[no_mangle]
#[panic_handler]
pub fn panic_fmt(_info: &core::panic::PanicInfo) -> ! {
	unsafe {
		panic(core::ptr::null(), 0u32);
	}
}

#[cfg(feature = "panic_with_msg")]
#[no_mangle]
#[panic_handler]
pub fn panic_fmt(info: &core::panic::PanicInfo) -> ! {

    struct Sink {
        buf: Vec<u8>,
        pos: usize
    }

    impl Sink {
        #[inline(always)]
        fn new(capacity: usize) -> Sink {
            let mut buf = Vec::with_capacity(capacity);
            buf.resize(capacity, 0);
            Sink {
                buf,
                pos: 0,
            }
        }

        #[inline(always)]
        fn reserve(&mut self, len: usize) -> &mut [u8] {
            let dst = &mut self.buf[self.pos..self.pos+len];
            self.pos += len;
            dst
        }

        #[inline(always)]
        fn write_u32(&mut self, val: u32) {
            LittleEndian::write_u32(self.reserve(4), val);
        }

        #[inline(always)]
        fn write_str(&mut self, bytes: &[u8]) {
            self.write_u32(bytes.len() as u32);
            self.reserve(bytes.len()).copy_from_slice(bytes)
        }
    }

    impl core::ops::Deref for Sink {
        type Target = [u8];
        fn deref(&self) -> &[u8] {
            &self.buf
        }
    }

    let msg = if let Some(fmt) = info.message() {
        format!("{}", fmt)
    } else {
        Default::default()
    };
    let (file, line, col) = if let Some(loc) = info.location() {
        (loc.file(), loc.line(), loc.column())
    } else {
        ("", 0, 0)
    };

    let mut sink = Sink::new(
        4 + msg.as_bytes().len() +		// len + [msg]
            4 + file.as_bytes().len() +		// len + [file]
            4 +								// line
            4								// col
    );
    sink.write_str(msg.as_bytes());
    sink.write_str(file.as_bytes());
    sink.write_u32(line);
    sink.write_u32(col);

    unsafe {
        panic(sink.as_ptr(), sink.len() as u32)
    }
}






// https://docs.rust-embedded.org/embedonomicon/smallest-no-std.html
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

// #[repr(transparent)]
// use core::alloc::Layout;
//

#[lang = "oom"]
#[no_mangle]
pub extern fn oom(_: core::alloc::Layout) -> ! {
    core::intrinsics::abort()
}
