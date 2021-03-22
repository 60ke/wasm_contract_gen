pub mod contract1 {
    use wasm_std::types::{U256, Address};
    use wasm2ct::types::*;
    use wasm2ct_derive::gen_contract;
    use wasm_std::String;
    use wasm2ct::types::{Stream, Sink};
    pub trait Interface {
        fn constructor(&mut self, _list: String) -> String;
        fn add_num(&mut self, a: u32, b: u32) -> u32;
        fn add_str(&mut self, a: String, b: String) -> String;
        fn ret_tuple(&mut self) -> (String, u32);
        fn ret_list(&mut self) -> [u8; 3];
        fn save_str(&mut self, a: String) -> bool;
        fn get_str(&mut self) -> String;
    }
    pub struct Contract<T: Interface> {
        pub inner: T,
    }
    impl<T: Interface> From<T> for Contract<T> {
        fn from(inner: T) -> Contract<T> {
            Contract { inner: inner }
        }
    }
    impl<T: Interface> Contract<T> {
        pub fn new(inner: T) -> Self {
            Contract { inner: inner }
        }
        pub fn instance(&self) -> &T {
            &self.inner
        }
    }
    impl<T: Interface> wasm2ct::ContractInterface for Contract<T> {
        #[allow(unused_mut)]
        #[allow(unused_variables)]
        fn call(&mut self, payload: &[u8]) -> Vec<u8> {
            let inner = &mut self.inner;
            if payload.len() < 4 {
                ::core::panicking::panic("Invalid abi invoke");
            }
            let method_id = ((payload[0] as u32) << 24)
                + ((payload[1] as u32) << 16)
                + ((payload[2] as u32) << 8)
                + (payload[3] as u32);
            let method_payload = &payload[4..];
            match method_id {
                3336898394u32 => {
                    let mut stream = wasm2ct::types::Stream::new(method_payload);
                    let ret = inner.add_num(
                        stream.pop::<u32>().expect("argument decoding failed"),
                        stream.pop::<u32>().expect("argument decoding failed"),
                    );
                    let mut sink = wasm2ct::types::Sink::new(1);
                    sink.push(ret);
                    sink.finalize_panicking()
                }
                3445704664u32 => {
                    let mut stream = wasm2ct::types::Stream::new(method_payload);
                    let ret = inner.add_str(
                        stream.pop::<String>().expect("argument decoding failed"),
                        stream.pop::<String>().expect("argument decoding failed"),
                    );
                    let mut sink = wasm2ct::types::Sink::new(1);
                    sink.push(ret);
                    sink.finalize_panicking()
                }
                2713154490u32 => {
                    let mut stream = wasm2ct::types::Stream::new(method_payload);
                    let ret = inner.ret_tuple();
                    let mut sink = wasm2ct::types::Sink::new(2);
                    sink.push(ret);
                    sink.finalize_panicking()
                }
                2066360337u32 => {
                    let mut stream = wasm2ct::types::Stream::new(method_payload);
                    let ret = inner.ret_list();
                    let mut sink = wasm2ct::types::Sink::new(1);
                    sink.push(ret);
                    sink.finalize_panicking()
                }
                1867203800u32 => {
                    let mut stream = wasm2ct::types::Stream::new(method_payload);
                    let ret =
                        inner.save_str(stream.pop::<String>().expect("argument decoding failed"));
                    let mut sink = wasm2ct::types::Sink::new(1);
                    sink.push(ret);
                    sink.finalize_panicking()
                }
                66506454u32 => {
                    let mut stream = wasm2ct::types::Stream::new(method_payload);
                    let ret = inner.get_str();
                    let mut sink = wasm2ct::types::Sink::new(1);
                    sink.push(ret);
                    sink.finalize_panicking()
                }
                _ => ::core::panicking::panic("Invalid method signature"),
            }
        }
        #[allow(unused_variables)]
        #[allow(unused_mut)]
        fn deploy(&mut self, payload: &[u8]) {
            let mut stream = wasm2ct::types::Stream::new(payload);
            self.inner
                .constructor(stream.pop::<String>().expect("argument decoding failed"));
        }
    }
    pub struct Outer {
        gas: Option<u64>,
        address: Address,
        value: Option<U256>,
    }
    impl Outer {
        pub fn new(address: Address) -> Self {
            Outer {
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
    impl Interface for Outer {
        fn constructor(&mut self, _list: String) -> String {
            #![allow(unused_mut)]
            #![allow(unused_variables)]
            ::core::panicking::panic("not implemented")
        }
        fn add_num(&mut self, a: u32, b: u32) -> u32 {
            #![allow(unused_mut)]
            #![allow(unused_variables)]
            let mut payload = Vec::with_capacity(4 + 2 * 32);
            payload.push((3336898394u32 >> 24) as u8);
            payload.push((3336898394u32 >> 16) as u8);
            payload.push((3336898394u32 >> 8) as u8);
            payload.push(3336898394u32 as u8);
            let mut sink = wasm2ct::types::Sink::new(2);
            sink.push(a);
            sink.push(b);
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
            let mut stream = wasm2ct::types::Stream::new(&result);
            stream.pop().expect("failed decode call output")
        }
        fn add_str(&mut self, a: String, b: String) -> String {
            #![allow(unused_mut)]
            #![allow(unused_variables)]
            let mut payload = Vec::with_capacity(4 + 2 * 32);
            payload.push((3445704664u32 >> 24) as u8);
            payload.push((3445704664u32 >> 16) as u8);
            payload.push((3445704664u32 >> 8) as u8);
            payload.push(3445704664u32 as u8);
            let mut sink = wasm2ct::types::Sink::new(2);
            sink.push(a);
            sink.push(b);
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
            let mut stream = wasm2ct::types::Stream::new(&result);
            stream.pop().expect("failed decode call output")
        }
        fn ret_tuple(&mut self) -> (String, u32) {
            #![allow(unused_mut)]
            #![allow(unused_variables)]
            let mut payload = Vec::with_capacity(4 + 0 * 32);
            payload.push((2713154490u32 >> 24) as u8);
            payload.push((2713154490u32 >> 16) as u8);
            payload.push((2713154490u32 >> 8) as u8);
            payload.push(2713154490u32 as u8);
            let mut sink = wasm2ct::types::Sink::new(0);
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
            let mut stream = wasm2ct::types::Stream::new(&result);
            stream.pop().expect("failed decode call output")
        }
        fn ret_list(&mut self) -> [u8; 3] {
            #![allow(unused_mut)]
            #![allow(unused_variables)]
            let mut payload = Vec::with_capacity(4 + 0 * 32);
            payload.push((2066360337u32 >> 24) as u8);
            payload.push((2066360337u32 >> 16) as u8);
            payload.push((2066360337u32 >> 8) as u8);
            payload.push(2066360337u32 as u8);
            let mut sink = wasm2ct::types::Sink::new(0);
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
            let mut stream = wasm2ct::types::Stream::new(&result);
            stream.pop().expect("failed decode call output")
        }
        fn save_str(&mut self, a: String) -> bool {
            #![allow(unused_mut)]
            #![allow(unused_variables)]
            let mut payload = Vec::with_capacity(4 + 1 * 32);
            payload.push((1867203800u32 >> 24) as u8);
            payload.push((1867203800u32 >> 16) as u8);
            payload.push((1867203800u32 >> 8) as u8);
            payload.push(1867203800u32 as u8);
            let mut sink = wasm2ct::types::Sink::new(1);
            sink.push(a);
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
            let mut stream = wasm2ct::types::Stream::new(&result);
            stream.pop().expect("failed decode call output")
        }
        fn get_str(&mut self) -> String {
            #![allow(unused_mut)]
            #![allow(unused_variables)]
            let mut payload = Vec::with_capacity(4 + 0 * 32);
            payload.push((66506454u32 >> 24) as u8);
            payload.push((66506454u32 >> 16) as u8);
            payload.push((66506454u32 >> 8) as u8);
            payload.push(66506454u32 as u8);
            let mut sink = wasm2ct::types::Sink::new(0);
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
            let mut stream = wasm2ct::types::Stream::new(&result);
            stream.pop().expect("failed decode call output")
        }
    }
}