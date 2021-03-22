use fixed_hash::{construct_fixed_hash,impl_fixed_hash_conversions};
use uint::construct_uint;


construct_uint! {
	pub struct U256(4);
}


construct_fixed_hash!{
    // 256数据hash类型
    pub struct H256(32);
}

construct_fixed_hash!{
    // Address hash
    pub struct H160(20);
}

pub type Address = H160;

impl_fixed_hash_conversions!(H256,H160);



impl From<U256> for H256{
    fn from(uint:U256) -> H256{
        let mut hash = H256::zero();
        uint.to_big_endian(hash.as_bytes_mut());
        hash
    }

}


impl<'a> From<&'a U256> for H256 {
    fn from(uint: &'a U256) -> H256 {
        let mut hash: H256 = H256::zero();
        uint.to_big_endian(hash.as_bytes_mut());
        hash
    }
}
//


impl From<H256> for U256{
    fn from(hash:H256) -> U256{
        U256::from(hash.as_bytes())
    }

}

impl<'a> From<&'a H256> for U256 {
	fn from(hash: &'a H256) -> U256 {
		U256::from(hash.as_bytes())
	}
}


