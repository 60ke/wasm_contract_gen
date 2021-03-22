pub mod token {}
pub mod token {}
pub mod token {
    use wasm_mid;
    use lazy_static;
    use wasm2ct::types::*;
    use wasm2ct_derive::gen_contract;
    use wasm_std::String;
    use wasm2ct::types::{Stream, Sink};
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
            ();
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
            let mut stream = pwasm_abi::eth::Stream::new(payload);
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
    pub struct Contract1;
    impl Interface for Contract1 {
        fn constructor(&mut self, list: String) -> String {
            list
        }
        fn add_num(&mut self, a: u32, b: u32) -> u32 {
            a + b
        }
        fn add_str(&mut self, a: String, b: String) -> String {
            (a + &b)
        }
        fn ret_tuple(&mut self) -> (String, u32) {
            (String::from("aaa"), 3)
        }
        fn ret_list(&mut self) -> [u8; 3] {
            [1, 2, 3]
        }
        fn save_str(&mut self, a: String) -> bool {
            storage_write(&STORAGE_KEY, a);
            true
        }
        fn get_str(&mut self) -> String {
            storage_read(&STORAGE_KEY)
        }
    }
    fn storage_write(key: &H256, a: String) {
        let mut sink = Sink::new(1);
        sink.push(a);
        let ret = sink.finalize_panicking();
        let num_len = ret.len() / 32;
        wasm_mid::write(key, &H256::from(U256::from(1)).into());
        wasm_mid::write(
            &h256_add(key, U256::from(1)),
            &H256::from(U256::from(num_len)).into(),
        );
        for idx in 0..num_len {
            let data = (&ret[32 * idx..32 * (idx + 1)]).as_ptr() as *const [u8; 32];
            unsafe {
                let tar: [u8; 32] = *data;
                wasm_mid::write(&h256_add(key, U256::from(2 + idx)), &tar);
            }
        }
    }
    fn convert_num(a: u32) -> Vec<u8> {
        let mut sink = Sink::new(1);
        sink.push(a);
        sink.finalize_panicking()
    }
    fn h256_add(a: &H256, b: U256) -> H256 {
        H256::from(U256::from(a) + b)
    }
    fn storage_read(key: &H256) -> String {
        let list = wasm_mid::read(&h256_add(key, U256::from(1)));
        let mut stream = Stream::new(&list[..]);
        let num_len: u32 = stream.pop().expect("x failed to pop");
        let mut tar = Vec::new();
        for idx in 0..num_len {
            let data = wasm_mid::read(&h256_add(key, U256::from(2 + idx)));
            tar.extend(&data);
        }
        let mut stream = Stream::new(&tar[..]);
        let ret: String = stream.pop().expect("string failed to pop");
        ret
    }
}