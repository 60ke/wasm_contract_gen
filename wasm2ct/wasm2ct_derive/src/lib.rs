use proc_macro::TokenStream;
use quote::quote;

mod utils;
// mod derive_log;
// mod common;
mod errors;
mod types;


// use derive_log::init_logger;
use types::{Abi, ContractCode,write_json_abi};






#[proc_macro_attribute]
pub fn gen_contract(args: TokenStream, item: TokenStream) -> TokenStream {
    // init_logger();
    let args = syn::parse_macro_input!(args as syn::LitBool);
    let item = syn::parse_macro_input!(item as syn::Item);
    let item_trait = match item {
        syn::Item::Trait(item_trait) => item_trait,
        _ => panic!("错误的Trait语法")
    };
    // derive_print!("{:?}",item_trait);
    let contract_code = ContractCode::new(args.value, item_trait.clone());
    // derive_print!("############# {:?}",contract_code);
    let abi = Abi::new(item_trait.clone());
    write_json_abi(&abi);
    let result = quote! {
        #contract_code
    };
    result.into()
}

