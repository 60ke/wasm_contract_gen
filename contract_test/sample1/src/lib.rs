#![no_std]
#![allow(non_snake_case)]


use wasm_std::types::{U256, Address};
use wasm_mid;
use wasm2ct::types::*;
use wasm2ct_derive::gen_contract;



use wee_alloc;
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[gen_contract(true)]
pub trait Interface{
    fn add_num(&mut self,a:u32,b:u32)->u32;
}

pub struct Contract1;
impl Interface for Contract1{
    fn add_num(&mut self,a:u32,b:u32)->u32{
        a + b
    }
}


use wasm2ct::ContractInterface;

#[no_mangle]
pub fn call() {
    let mut endpoint = Contract::new(Contract1{});
    // Read http://solidity.readthedocs.io/en/develop/abi-spec.html#formal-specification-of-the-encoding for details
    wasm_mid::ret(&endpoint.call(&wasm_mid::input()));
}

#[no_mangle]
pub fn deploy() {
    let mut endpoint = Contract::new(Contract1{});
    endpoint.deploy(&wasm_mid::input());
}