use byteorder::{BigEndian, ByteOrder};
use crate::types::*;

// 将u32,u64,等数据类型转化为H256 用于事件存储
pub trait Convert2Event {
	fn convert2event(&self) -> H256;
}

impl Convert2Event for u32 {
	fn convert2event(&self) -> H256 {
		let mut result = H256::zero();
		BigEndian::write_u32(&mut result.as_mut()[28..32], *self);
		result
	}
}

impl Convert2Event for u64 {
	fn convert2event(&self) -> H256 {
		let mut result = H256::zero();
		BigEndian::write_u64(&mut result.as_mut()[24..32], *self);
		result
	}
}

impl Convert2Event for i64 {
	fn convert2event(&self) -> H256 {
		let mut result = H256::zero();
		BigEndian::write_i64(&mut result.as_mut()[24..32], *self);
		result
	}
}

impl Convert2Event for i32 {
	fn convert2event(&self) -> H256 {
		let mut result = H256::zero();
		BigEndian::write_i32(&mut result.as_mut()[28..32], *self);
		result
	}
}


impl Convert2Event for bool {
	fn convert2event(&self) -> H256 {
		let mut result = H256::zero();
		result.as_mut()[32] = if *self { 1 } else { 0 };
		result
	}
}

impl Convert2Event for U256 {
	fn convert2event(&self) -> H256 {
		let mut result = H256::zero();
		self.to_big_endian(result.as_mut());
		result
	}
}

impl Convert2Event for H256 {
	fn convert2event(&self) -> H256 {
		self.clone()
	}
}

impl Convert2Event for Address {
	fn convert2event(&self) -> H256 {
		(*self).into()
	}
}
