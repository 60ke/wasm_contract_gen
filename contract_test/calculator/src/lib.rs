#![no_std]
#![allow(non_snake_case)]
#![feature(proc_macro_hygiene)]


use wasm_std::types::{U256, Address};
use wasm_mid;
use lazy_static;
use wasm2ct::types::*;
use wasm2ct_derive::gen_contract;
use wasm_std::String;
use wasm2ct::types::{Stream,Sink};

use wee_alloc;
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

lazy_static::lazy_static!{
    static ref STORAGE_KEY: H256 = H256::from(
            [2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
    );
}
#[gen_contract(true)]
pub trait Interface{
    fn constructor(&mut self,_list:String)->String;
    fn add_num(&mut self,a:u32,b:u32)->u32;
    fn add_str(&mut self,a:String,b:String)->String;
    fn ret_tuple(&mut self)->(String,u32);
    fn ret_list(&mut self)->[u8;3];
    fn save_str(&mut self,a:String)->bool;
    fn get_str(&mut self)->String;
}

pub struct Contract1;
impl Interface for Contract1{
    fn constructor(&mut self,list:String)->String{
        list
    }
    fn add_num(&mut self,a:u32,b:u32)->u32{
        a + b
    }
    fn add_str(&mut self,a:String,b:String)->String{
        a  + &b
    }
    fn ret_tuple(&mut self)->(String,u32){
        (String::from("aaa"),3)
    }
    fn ret_list(&mut self)->[u8;3]{
        [1,2,3]
    }
    fn save_str(&mut self,a:String)->bool{
        storage_write(&STORAGE_KEY, a);
        true
    }
    fn get_str(&mut self)->String{
        storage_read(&STORAGE_KEY)
    }
}

// fn storage_key(address:&Address) -> H256{
//     let mut key:H256 = H256::from(*address);
//     key.as_bytes_mut()[0] = 1;
//     key
// }
fn storage_write(key: &H256,a:String){
    let mut sink = Sink::new(1);
    sink.push(a);
    let ret = sink.finalize_panicking();
    let num_len = ret.len() /32;
    // 存储sdk版本
    wasm_mid::write(key,&H256::from(U256::from(1)).into());
    // 存储数据长度
    wasm_mid::write(&h256_add(key, U256::from(1)),&H256::from(U256::from(num_len)).into());
    for idx in 0..num_len{
        let data = (&ret[32*idx..32*(idx+1)]).as_ptr() as *const [u8; 32];
        unsafe {
            let tar:[u8;32] = *data;
            wasm_mid::write(&h256_add(key, U256::from(2+idx)),&tar);
        }
        
    }
}
// fn convert_num(a:u32)->Vec<u8>{
//     let mut sink = Sink::new(1);
//     sink.push(a);
//     sink.finalize_panicking()
// }
fn h256_add(a:&H256,b:U256)->H256{
    H256::from(U256::from(a) + b)
}
fn storage_read(key: &H256)->String{
    // 读取数据长度
    let list = wasm_mid::read(&h256_add(key, U256::from(1)));
    let mut stream = Stream::new(&list[..]);
    let num_len:u32 = stream.pop().expect("x failed to pop");
    let mut tar = Vec::new();
    for idx in 0..num_len{
        let data = wasm_mid::read(&h256_add(key, U256::from(2+idx)));
        tar.extend(&data);
    }
    let mut stream = Stream::new(&tar[..]);
    let ret:String = stream.pop().expect("string failed to pop");
    ret
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