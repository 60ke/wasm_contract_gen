#![cfg_attr(not(feature="std"), no_std)]
#![cfg_attr(feature="strict", deny(unused))]


extern crate byteorder;
extern crate wasm_std;

#[cfg(test)]
#[cfg_attr(all(test, feature = "std"), macro_use)]
extern crate hex_literal;

#[cfg(not(feature="std"))]
#[allow(unused)]
#[macro_use] extern crate alloc;


pub mod types;
mod event;
mod codec;

mod lib {

	mod core {
		#[cfg(feature = "std")]
		pub use std::*;
		#[cfg(not(feature = "std"))]
		pub use core::*;
	}

	pub use self::core::{cmp, iter, mem, ops, slice, str,i8, i16, i32, i64, isize,u8, u16, u32, u64, usize};
	pub use self::core::cell::{Cell, RefCell};
	pub use self::core::clone::{self, Clone};
	pub use self::core::convert::{self, From, Into};
	pub use self::core::default::{self, Default};
	pub use self::core::fmt::{self, Debug, Display};
	pub use self::core::marker::{self, PhantomData};
	pub use self::core::option::{self, Option};
	pub use self::core::result::{self, Result};

	#[cfg(feature = "std")]
	pub use std::borrow::{Cow, ToOwned};
	#[cfg(not(feature = "std"))]
	pub use alloc::borrow::{Cow, ToOwned};

	#[cfg(feature = "std")]
	pub use std::string::String;
	#[cfg(not(feature = "std"))]
	pub use alloc::string::{String, ToString};

	#[cfg(feature = "std")]
	pub use std::vec::Vec;
	#[cfg(not(feature = "std"))]
	pub use alloc::vec::Vec;

	#[cfg(feature = "std")]
	pub use std::boxed::Box;
	#[cfg(not(feature = "std"))]
	pub use alloc::boxed::Box;
}

// 合约接口
pub trait ContractInterface {
	/// 合约调用
	fn call(&mut self, payload: &[u8]) -> crate::lib::Vec<u8>;

	/// 合约部署
	fn deploy(&mut self, payload: &[u8]);
}
