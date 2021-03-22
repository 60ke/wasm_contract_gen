function_selector: constructor(string)
0xd52b…e80e
3576392589
function_selector: change_list()
0xe49c…20b6
3835443950
Args { ename: "TokenEndpoint", cname: Some("TokenClient") }
#![feature(prelude_import)]
#![no_std]
#![allow(non_snake_case)]
#![feature(proc_macro_hygiene)]
#[prelude_import]
use core::prelude::v1::*;
#[macro_use]
extern crate core;
#[macro_use]
extern crate compiler_builtins;
extern crate wasm_std;
extern crate wasm_mid;
extern crate wasm2ct;
extern crate sdk_derive;
use wasm_std::types::{U256, Address};
pub mod token {
    use wasm_mid;
    use lazy_static;
    use wasm2ct::types::*;
    use wasm2ct;
    use sdk_derive::ewasm_abi;
    #[allow(missing_copy_implementations)]
    #[allow(non_camel_case_types)]
    #[allow(dead_code)]
    struct STORAGE_KEY {
        __private_field: (),
    }
    #[doc(hidden)]
    static STORAGE_KEY: STORAGE_KEY = STORAGE_KEY {
        __private_field: (),
    };
    impl ::lazy_static::__Deref for STORAGE_KEY {
        type Target = H256;
        fn deref(&self) -> &H256 {
            #[inline(always)]
            fn __static_ref_initialize() -> H256 {
                H256::from([
                    2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0,
                ])
            }
            #[inline(always)]
            fn __stability() -> &'static H256 {
                static LAZY: ::lazy_static::lazy::Lazy<H256> = ::lazy_static::lazy::Lazy::INIT;
                LAZY.get(__static_ref_initialize)
            }
            __stability()
        }
    }
    impl ::lazy_static::LazyStatic for STORAGE_KEY {
        fn initialize(lazy: &Self) {
            let _ = &**lazy;
        }
    }
    pub trait Interface {
        fn constructor(&mut self, _list: String);
        fn change_list(&mut self) -> String;
    }
    #[allow(non_snake_case)]
    mod ewasm_sdk_impl_Interface {
        extern crate wasm_mid;
        extern crate wasm2ct;
        use wasm2ct::types::{H160, H256, U256, Address, Vec, String};
        use super::Interface;
        pub struct TokenEndpoint<T: Interface> {
            pub inner: T,
        }
        impl<T: Interface> From<T> for TokenEndpoint<T> {
            fn from(inner: T) -> TokenEndpoint<T> {
                TokenEndpoint { inner: inner }
            }
        }
        impl<T: Interface> TokenEndpoint<T> {
            pub fn new(inner: T) -> Self {
                TokenEndpoint { inner: inner }
            }
            pub fn instance(&self) -> &T {
                &self.inner
            }
        }
        impl<T: Interface> wasm2ct::ewasm::EndpointInterface for TokenEndpoint<T> {
            #[allow(unused_mut)]
            #[allow(unused_variables)]
            fn dispatch(&mut self, payload: &[u8]) -> Vec<u8> {
                let inner = &mut self.inner;
                if payload.len() < 4 {
                    ::core::panicking::panic("非法函数调用");
                }
                let method_id = ((payload[0] as u32) << 24)
                    + ((payload[1] as u32) << 16)
                    + ((payload[2] as u32) << 8)
                    + (payload[3] as u32);
                let method_payload = &payload[4..];
                match method_id {
                    3835443950 => {
                        if wasm_mid::value() > 0.into() {
                            ::core::panicking::panic("non-payable 的value值不能大于0");
                        }
                        let mut stream = wasm2ct::ewasm::Stream::new(method_payload);
                        let result = inner.change_list();
                        let mut sink = wasm2ct::ewasm::Sink::new(1);
                        sink.push(result);
                        sink.finalize_panicking()
                    }
                    _ => ::core::panicking::panic("函数不存在"),
                }
            }
            #[allow(unused_variables)]
            #[allow(unused_mut)]
            fn dispatch_ctor(&mut self, payload: &[u8]) {
                if wasm_mid::value() > 0.into() {
                    ::core::panicking::panic("non-payable 的value值不能大于0");
                }
                let mut stream = wasm2ct::ewasm::Stream::new(payload);
                self.inner
                    .constructor(stream.pop::<String>().expect("参数解码失败"));
            }
        }
        pub struct TokenClient {
            gas: Option<u64>,
            address: Address,
            value: Option<U256>,
        }
        impl TokenClient {
            pub fn new(address: Address) -> Self {
                TokenClient {
                    gas: None,
                    address: address,
                    value: None,
                }
            }
            pub fn gas(mut self, gas: u64) -> Self {
                self.gas = Some(gas);
                self
            }
            pub fn value(mut self, val: U256) -> Self {
                self.value = Some(val);
                self
            }
        }
        impl Interface for TokenClient {
            fn constructor(&mut self, _list: String) {
                #![allow(unused_mut)]
                #![allow(unused_variables)]
                ::core::panicking::panic("not implemented")
            }
            fn change_list(&mut self) -> String {
                #![allow(unused_mut)]
                #![allow(unused_variables)]
                let mut payload = Vec::with_capacity(4 + 0 * 32);
                payload.push((3835443950 >> 24) as u8);
                payload.push((3835443950 >> 16) as u8);
                payload.push((3835443950 >> 8) as u8);
                payload.push(3835443950 as u8);
                let mut sink = wasm2ct::ewasm::Sink::new(0);
                sink.drain_to(&mut payload);
                let mut result = [0u8; 32];
                wasm_mid::call(
                    self.gas.unwrap_or(200000),
                    &self.address,
                    self.value.clone().unwrap_or(U256::zero()),
                    &payload,
                    &mut result[..],
                )
                .expect("Call failed; todo: allow handling inside contracts");
                let mut stream = wasm2ct::ewasm::Stream::new(&result);
                stream.pop().expect("failed decode call output")
            }
        }
    }
    pub use self::ewasm_sdk_impl_Interface::TokenEndpoint;
    pub use self::ewasm_sdk_impl_Interface::TokenClient;
    pub struct Contract;
    impl Interface for Contract {
        fn constructor(&mut self, list: String) {
            wasm_mid::write(&STORAGE_KEY, list.as_bytes());
        }
        fn change_list(&mut self) -> String {
            let sender: Address = wasm_mid::address();
            let tar = U256::from_big_endian(&wasm_mid::read(&STORAGE_KEY));
            wasm_mid::write(&STORAGE_KEY, &(tar - 1).into());
            tar - 1
        }
    }
    fn storage_key(address: &Address) -> H256 {
        let mut key: H256 = H256::from(*address);
        key.as_bytes_mut()[0] = 1;
        key
    }
}
use wasm2ct::ewasm::EndpointInterface;
#[no_mangle]
pub fn call() {
    let mut endpoint = token::TokenEndpoint::new(token::Contract {});
    wasm_mid::ret(&endpoint.dispatch(&wasm_mid::input()));
}
#[no_mangle]
pub fn deploy() {
    let mut endpoint = token::TokenEndpoint::new(token::Contract {});
    endpoint.dispatch_ctor(&wasm_mid::input());
}
