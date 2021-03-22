// 宏参数,inner_call用于生成合约内部调用的接口
// outer_call接口用于被其它合约调用

// use crate::errors::Error;
use proc_macro2::{Ident, Span, TokenStream};
use crate::utils::{gen_trait_part,gen_call_part,gen_deploy_part,gen_constructor_part,gen_event_part,gen_other_part,gen_func_abi,gen_constructor_abi,gen_event_abi};
use fixed_hash::construct_fixed_hash;
use quote::{quote, TokenStreamExt};
use serde::{Serialize};
// use serde_json;
// use serde_derive;

construct_fixed_hash! {
	/// Uninterpreted 32 byte (256 bit) large hash type.
	pub struct H256(32);
}

pub type TokenVec = Vec<proc_macro2::TokenStream>;
// pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub struct DeriveArgs {
    pub inner_call_name: String,
    pub outer_call_name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ContractCode {
    inner_call_part: ContractInnerCall,
    outer_call_part: Option<ContractOuterCall>,
}

#[derive(Debug, Clone)]
pub struct ContractInnerCall {
    inner_methods: ContractInnerMethod,
}

#[derive(Debug, Clone)]
pub struct ContractOuterCall {
    outer_methods: ContractOuterMethod,
}

#[derive(Debug, Clone)]
pub struct ContractInnerMethod {
    trait_part: TokenStream,
    call_part: TokenStream,
    deploy_part: TokenStream,
}

#[derive(Debug, Clone)]
pub struct ContractOuterMethod {
    constructor_part: TokenStream,
    event_part: TokenStream,
    other_part: TokenStream,
}

#[derive(Debug, Clone)]
pub struct Interface{
    name:Ident,
    constructor:Vec<syn::TraitItemMethod>,
    event:Vec<syn::TraitItemMethod>,
    others:Vec<syn::TraitItemMethod>,
}


impl Interface{
    pub fn parse(item_trait: syn::ItemTrait)->Self{
        let mut constructor_items = Vec::new();
        let mut event_items = Vec::new();
        let mut other_items = Vec::new();


        // 未考虑具有constructor,event的情况
        // 是否支持constructor多态,以及多个event?
        for item in item_trait.items{
            match item {
                syn::TraitItem::Method(trait_item_method) => {
                    if trait_item_method.sig.ident == "constructor"{
                        constructor_items.push(trait_item_method);
                    }else if  method_is_event(&trait_item_method){
                        event_items.push(trait_item_method)
                    }else {
                        other_items.push(trait_item_method)
                    }
                },
                _ => panic!("trait method parse error")
            };
        }
        Interface{
            name:item_trait.ident,
            constructor:constructor_items,
            event:event_items,
            others:other_items,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Abi(pub Vec<AbiEntry>);
#[derive(Serialize, Debug)]
#[serde(tag = "type")]
pub enum AbiEntry {
    #[serde(rename = "event")]
    Event(EventAbi),
    #[serde(rename = "constructor")]
    Constructor(ConstructorAbi),    
    #[serde(rename = "function")]
    Others(FunctionAbi),
}
#[derive(Serialize, Debug)]
pub struct EventAbi{
    pub name: String,
    pub inputs: Vec<EventInput>,    
}

#[derive(Serialize, Debug)]
pub struct EventInput {
    #[serde(rename = "name")]
    pub argname: String,
    #[serde(rename = "type")]
    pub argtype: String,
    pub indexed: bool,
}
#[derive(Serialize, Debug)]
pub struct FunctionAbi{
    pub name: String,
    pub inputs: Vec<Argument>,
    pub outputs: Vec<Argument>,    
}
#[derive(Serialize, Debug)]
pub struct ConstructorAbi{
    pub inputs: Vec<Argument>,    
}


#[derive(Serialize, Debug)]
pub struct Argument {
    #[serde(rename = "name")]
    pub argname: String,
    #[serde(rename = "type")]
    pub argtype: String,
}

impl Abi {
    pub fn new(item_trait: syn::ItemTrait) -> Abi {
        let mut abi_entrys = Vec::new();
        let interface = Interface::parse(item_trait);
        for func in interface.others{
            let funcabi = gen_func_abi(func);
            abi_entrys.push(AbiEntry::Others(funcabi));            
        }

        for func in interface.event{
            let eventabi = gen_event_abi(func);
            abi_entrys.push(AbiEntry::Event(eventabi));            
        }
        
        for func in interface.constructor{
            let consabi = gen_constructor_abi(func);
            abi_entrys.push(AbiEntry::Constructor(consabi));            
        }         
        
        let abi = Abi(abi_entrys);
        abi
    }

}


pub fn write_json_abi(abi: &Abi){
	use std::{env, fs, path};
    use std::io::Write;
	let target = {
		let mut target =
			path::PathBuf::from(env::var("CARGO_TARGET_DIR").unwrap_or(".".to_owned()));
		target.push("target");
		target.push("json");
		fs::create_dir_all(&target).unwrap();
		target.push("abi.json");
		target
	};

	let mut f =
		fs::File::create(target).map_err(|err| println!("{:?}",err)).unwrap();


	let tar = serde_json::to_string_pretty(&abi)
		.map_err(|err| println!("{:?}",err)).unwrap();

    f.write(tar.as_bytes()).unwrap();
	// Ok(())
}



pub fn method_is_event(method:&syn::TraitItemMethod)->bool{
    if method.attrs.len() == 0{
        return false
    }else if method.attrs[0].path.segments.first().unwrap().ident == "event"{
        return true
    }else {
        panic!("函数限定 {:?} 不合法\n", method.attrs)
    }
}


impl ContractInnerMethod {
    fn gen(item_trait: syn::ItemTrait) -> ContractInnerMethod {

        let interface = Interface::parse(item_trait.clone());
        // println!("interface--------------  {:#?}",interface);
        let trait_part = gen_trait_part(item_trait.clone());
        let call_part = gen_call_part(interface.others.clone());
        let deploy_part = gen_deploy_part( interface.constructor.clone());
        ContractInnerMethod {
            trait_part,
            call_part,
            deploy_part
        }
    }
}



impl ContractOuterMethod {
    fn gen(item_trait: syn::ItemTrait) -> ContractOuterMethod {

        let interface = Interface::parse(item_trait.clone());
        let constructor_part = gen_constructor_part(interface.constructor.clone());
        let event_part = gen_event_part(interface.event.clone());        // println!("callpart {:?}",call_part);
        let other_part = gen_other_part( interface.others.clone());
        ContractOuterMethod {
            constructor_part,
            event_part,
            other_part
        }
    }
}




impl ContractCode {
    pub fn new(args_value: bool, item_trait: syn::ItemTrait) -> ContractCode {
        let inner_methods = ContractInnerMethod::gen( item_trait.clone());
        let inner_call_part = ContractInnerCall {
            inner_methods,
        };
        let outer_call_part;
        if args_value == true {
            outer_call_part = Some(ContractOuterCall {
                outer_methods: ContractOuterMethod::gen(item_trait.clone()),
            });
        } else { outer_call_part = None; }
        let contract_code = ContractCode {
            inner_call_part,
            outer_call_part,
        };
        contract_code
    }
}

impl quote::ToTokens for ContractCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let inner_call_part = &self.inner_call_part;
        match &self.outer_call_part {
            Some(outer_call_part) =>
                tokens.append_all(
                    quote!(
                #inner_call_part
                #outer_call_part
			        )
                ),
            _ =>
                tokens.append_all(
                    quote!(
                #inner_call_part
			)),
        }
    }
}

impl quote::ToTokens for ContractInnerCall {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let inner_methods = &self.inner_methods;
        let trait_part = &inner_methods.trait_part;
        let call_part = &inner_methods.call_part;
        let deploy_part = &inner_methods.deploy_part;
        let contract_ident = syn::Ident::new("Contract", Span::call_site());
        let interface_ident = Ident::new("Interface", Span::call_site());
        tokens.append_all(
            quote! {
                #trait_part
                pub struct #contract_ident<T: #interface_ident> {
                    pub inner: T,
                }

                impl<T: #interface_ident> From<T> for #contract_ident<T> {
                    fn from(inner: T) -> #contract_ident<T> {
                        #contract_ident {
                            inner: inner,
                        }
                    }
                }

                impl<T: #interface_ident> #contract_ident<T> {
                    pub fn new(inner: T) -> Self {
                        #contract_ident {
                            inner: inner,
                        }
                    }

                    pub fn instance(&self) -> &T {
                        &self.inner
                    }
                }
                impl<T: #interface_ident> wasm2ct::ContractInterface for #contract_ident<T> {
                    #[allow(unused_mut)]
                    #[allow(unused_variables)]
                    fn call(&mut self, payload: &[u8]) -> Vec<u8> {
                        let inner = &mut self.inner;
                        if payload.len() < 4 {
                            panic!("Invalid abi invoke");
                        }
                        let method_id = ((payload[0] as u32) << 24)
                            + ((payload[1] as u32) << 16)
                            + ((payload[2] as u32) << 8)
                            + (payload[3] as u32);
                        // println!("method_id {:?}",method_id);

                        let method_payload = &payload[4..];

                        match method_id {
                            #call_part
                            _ => panic!("Invalid method signature"),
				        }
				    }
                    #[allow(unused_variables)]
                    #[allow(unused_mut)]
                    fn deploy(&mut self, payload: &[u8]) {
                        #deploy_part
                    }

                }
            }
        );
    }
}

impl quote::ToTokens for ContractOuterCall {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let outer_methods = &self.outer_methods;
        let constructor_part = &outer_methods.constructor_part;
        let event_part = &outer_methods.event_part;
        let other_part = &outer_methods.other_part;
        let outer_ident = syn::Ident::new("Outer", Span::call_site());
        let interface_ident = Ident::new("Interface", Span::call_site());
        tokens.append_all(quote! {
            pub struct #outer_ident {
                gas: Option<u64>,
                address: Address,
                value: Option<U256>,
            }

            impl #outer_ident {
                pub fn new(address: Address) -> Self {
                    #outer_ident {
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

            impl #interface_ident for #outer_ident {
                #constructor_part
                #event_part
                #other_part
            }
        })
    }
}




