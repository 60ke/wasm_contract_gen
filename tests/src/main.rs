// #![cfg_attr(not(feature = "std"),no_std)]
// // #![cfg_attr(not(feature = "std"),no_std)]
// #![cfg_attr(feature = "tiny",deny(unused))]
//
//
// #[cfg(feature="std")]
// use std::println;
//
use std::println;

use wasm2ct_derive::gen_contract;
use wasm_std::types::*;
// use wasm_std::Vec;


// lazy_static::lazy_static! {
//         static ref STORAGE_KEY: H256 =
//             H256::from(
//                 [2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
//             );
//     }
// false无问题
#[gen_contract(true)]
pub trait Interface {
    // The constructor
    // fn constructor(&mut self, _total_supply: u8);
    // Total amount of tokens
    // fn constructor(&mut self,a:u8);
    fn add(&mut self,a:u32,b:u32) -> u32;
    fn sub(&mut self,a:u32,b:u32) -> u32;
    // What is the balance of a particular account?
}


pub struct Contract1;

impl Interface for Contract1{
    // fn constructor(&mut self,a:u8){
    //     a;
    // }
    fn add(&mut self,a:u32,b:u32) ->u32{
        a + b
    }
    fn sub(&mut self,a:u32,b:u32) ->u32{
        a + b
    }
}



use hex;
use wasm2ct::ContractInterface;
// use wasm2ct::types::Stream
use rustc_hex::FromHex;
fn main() {
    let input = "3ad14af300000000000000000000000000000000000000000000000000000000000000170000000000000000000000000000000000000000000000000000000000000018";
    // let decoded2 = input.from_hex::<Vec<u8>>().unwrap();
    let decoded = hex::decode(input).expect("Decoding failed");
    println!("{:?}",decoded);
    // println!("{:?}",decoded2);
    let mut contract = Contract::new(Contract1{});
    let b = contract.call(&decoded);
    println!("{:?}",b);
}

