#![no_std]

extern crate wasm_std;
extern crate wasm_mid as ext;

use wasm_std::Vec;


#[no_mangle]
pub fn call() {
    ext::ret(&{
        let mut data = Vec::with_capacity(40);
        data.extend_from_slice(&[5u8; 40][..]);
        data
    });
}

// 合约本身:Struct
// 合约方法:Trait
//

pub struct Contract;