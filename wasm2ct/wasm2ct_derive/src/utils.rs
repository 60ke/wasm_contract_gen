use quote::quote;
// use proc_macro2;
use proc_macro2::{Span};
// parse signature to function_name
use tiny_keccak::{Hasher,Keccak};
use crate::types::{H256,TokenVec,FunctionAbi,Argument,ConstructorAbi,EventAbi,EventInput};
// use std::future::get_context;
use byteorder::{BigEndian, ByteOrder};
use log::{debug};



pub fn gen_trait_part(item_trait:syn::ItemTrait)->proc_macro2::TokenStream{
    // derive_print!("gen_trait_part start----------------------------------------\n");
    let trait_part:TokenVec = item_trait.clone().items.into_iter().map(|item| {
        match item {
            syn::TraitItem::Method(func) => {
                handle_trait_func(func)
            }
            _ => panic!("函数解析错误!")
        }
    }).collect();
    // derive_print!("display ident {:?}",item_trait.ident.clone());
    let item_trait_ident = syn::Ident::new(&item_trait.ident.to_string(),Span::call_site());
    quote! {
        pub trait #item_trait_ident{
            #(#trait_part)*
        }
    }
}

pub fn gen_call_part(call_items:Vec<syn::TraitItemMethod>)->proc_macro2::TokenStream{
    // derive_print!("gen_call_part start----------------------------------------\n");
    let call_part:TokenVec = call_items.into_iter().map(|func| {
            handle_call_func(func)
        }
    ).collect();
    quote! {#(#call_part)*}
}




pub fn gen_deploy_part(constructor_items:Vec<syn::TraitItemMethod>)->proc_macro2::TokenStream{
    // derive_print!("gen_deploy_part start----------------------------------------\n");
    let deploy_part:TokenVec = constructor_items.into_iter().map(|func| {
        handle_deploy_func(func)
    }).collect();
    quote! {#(#deploy_part)*}
}



// for outer call


pub fn gen_constructor_part(constructor_items:Vec<syn::TraitItemMethod>)->proc_macro2::TokenStream{
    // derive_print!("gen_constructor_part start----------------------------------------\n");
    let constructor_part:TokenVec = constructor_items.into_iter().map(|func| {
        handle_outer_constructor(func)
    }).collect();
    quote! {#(#constructor_part)*}
}

pub fn gen_event_part(constructor_items:Vec<syn::TraitItemMethod>)->proc_macro2::TokenStream{
    // derive_print!("gen_event_part start----------------------------------------\n");
    let event_part:TokenVec = constructor_items.into_iter().map(|func| {
        handle_outer_event(func)
    }).collect();
    quote! {#(#event_part)*}
}

pub fn gen_other_part(constructor_items:Vec<syn::TraitItemMethod>)->proc_macro2::TokenStream{
    // derive_print!("gen_event_part start----------------------------------------\n");
    let other_part:TokenVec = constructor_items.into_iter().map(|func| {
        handle_outer_other(func)
    }).collect();
    quote! {#(#other_part)*}
}


fn handle_outer_constructor(func: syn::TraitItemMethod) -> proc_macro2::TokenStream {
    let args = func.sig.inputs.iter().filter_map(|arg| match arg {
        syn::FnArg::Typed(arg_captured) => {
            let pat = &arg_captured.pat;
            let ty = &arg_captured.ty;
            Some(quote!{#pat: #ty})
        }
        _ => None,
    });
    // println!("handle_outer_constructor: {:?}",func.sig.ident);
    match &func.sig.output {
        syn::ReturnType::Type(_, ref output) => {
            quote!{
				fn constructor(&mut self, #(#args),*) -> #output {
                    #![allow(unused_mut)]
                    #![allow(unused_variables)]
                    unimplemented!()
				}
			}
        },
        syn::ReturnType::Default => {
            quote!{
				fn constructor(&mut self, #(#args),*) {
                    #![allow(unused_mut)]
                    #![allow(unused_variables)]
                    unimplemented!()
				}
			}
        }
    }
}




fn handle_outer_event(func: syn::TraitItemMethod) -> proc_macro2::TokenStream {
    let args = func.sig.inputs.iter().filter_map(|arg| match arg {
        syn::FnArg::Typed(arg_captured) => {
            let pat = &arg_captured.pat;
            let ty = &arg_captured.ty;
            Some(quote!{#pat: #ty})
        }
        _ => None,
    });
    let func_ident = &func.sig.ident;
    match &func.sig.output {
        syn::ReturnType::Type(_, ref output) => {
            quote!{
				fn #func_ident(&mut self, #(#args),*) -> #output {
                    #![allow(unused_variables)]
                    panic!("不能从外部调用合约事件");
				}
			}
        },
        syn::ReturnType::Default => {
            quote!{
				fn #func_ident(&mut self, #(#args),*) {
                    #![allow(unused_variables)]
                    panic!("不能从外部调用合约事件");
				}
			}
        }
    }
}



fn handle_outer_other(func: syn::TraitItemMethod) -> proc_macro2::TokenStream {
    let args = func.sig.inputs.iter().filter_map(|arg| match arg {
        syn::FnArg::Typed(arg_captured) => {
            let pat = &arg_captured.pat;
            let ty = &arg_captured.ty;
            Some(quote!{#pat: #ty})
        }
        _ => None,
    });
    // let args = vec![quote!{}];

    let func_hash = gen_func_hash(&func.sig);
    let func_ident = func.sig.ident.clone();
    let func_input_pats = parse_func_input(func.sig.clone()).0;
    let argument_push:Vec<proc_macro2::TokenStream> = func_input_pats.iter().map(|pat| quote! { sink.push(#pat); }).collect();
    let func_input_litint = syn::Lit::Int(syn::LitInt::new(
        &(func_input_pats.len() as u64).to_string(),
        Span::call_site()
    ));
    let result_instance = match func.sig.output {
        syn::ReturnType::Default => quote!{
			let mut result = Vec::new();
		},
        syn::ReturnType::Type(_, _) => quote!{
			let mut result = [0u8; 32];
		},
    };
    let result_pop = match func.sig.output {
        syn::ReturnType::Default => None,
        syn::ReturnType::Type(_, _) => Some(
            quote!{
                let mut stream = wasm2ct::types::Stream::new(&result);
                stream.pop().expect("failed decode call output")
			}
        ),
    };
    let func_content = quote! {
        #![allow(unused_mut)]
        #![allow(unused_variables)]
        let mut payload = Vec::with_capacity(4 + #func_input_litint * 32);
        payload.push((#func_hash >> 24) as u8);
        payload.push((#func_hash >> 16) as u8);
        payload.push((#func_hash >> 8) as u8);
        payload.push(#func_hash as u8);

        let mut sink = wasm2ct::types::Sink::new(#func_input_litint);
        #(#argument_push)*

        sink.drain_to(&mut payload);

        #result_instance

        wasm_mid::call(self.gas.unwrap_or(200000), &self.address, self.value.clone().unwrap_or(U256::zero()), &payload, &mut result[..])
            .expect("Call failed; todo: allow handling inside contracts");

        #result_pop
    };
    // let func_content = quote!{};

    match func.sig.output {
        syn::ReturnType::Type(_, ref output) => {
            quote!{
				fn #func_ident(&mut self, #(#args),*) -> #output {
                    #func_content
				}
			}
        },
        syn::ReturnType::Default => {
            quote!{
				fn #func_ident(&mut self, #(#args),*) {
				    #func_content
				}
			}
        }
    }
}






//
//
//
// 放弃鸡肋的payable与constant限定,只对event进行解析
fn handle_trait_func(func: syn::TraitItemMethod) -> proc_macro2::TokenStream {
    // derive_print!("start handle_trait_func----------- {:?}",func);
    if func.attrs.len() == 0 {
        return quote! {#func};
    } else {
        if func.attrs[0].path.segments.first().unwrap().ident != "event" {
            panic!("函数限定 {:?} 不合法\n", func.attrs)
        }
        let mut event = func.clone();

        event.attrs = vec![];
        parse_event_trait(event.sig)
    }
}
//
//
// // TraitItemMethod { attrs: [], sig: Signature { constness: None, asyncness: None, unsafety: None, abi: None, fn_token: Fn, ident: Ident { ident: "add", span: # 0 bytes(598..601) }, generics: Generics { lt_token: None, params: [], gt_token: None, where_clause: None }, paren_token: Paren, inputs: [Receiver(Receiver { attrs: [], reference: Some((And, None)), mutability: Some(Mut), self_token: SelfValue }), Comma, Typed(PatType { attrs: [], pat: Ident(PatIdent { attrs: [], by_ref: None, mutability: None, ident: Ident { ident: "a", span: # 0 bytes(612..613) }, subpat: None }), colon_token: Colon, ty: Path(TypePath { qself: None, path: Path { leading_colon: None, segments: [PathSegment { ident: Ident { ident: "U256", span: # 0 bytes(614..618) }, arguments: None }] } }) }), Comma, Typed(PatType { attrs: [], pat: Ident(PatIdent { attrs: [], by_ref: None, mutability: None, ident: Ident { ident: "b", span: # 0 bytes(619..620) }, subpat: None }), colon_token: Colon, ty: Path(TypePath { qself: None, path: Path { leading_colon: None, segments: [PathSegment { ident: Ident { ident: "U256", span: # 0 bytes(621..625) }, arguments: None }] } }) })], variadic: None, output: Type(RArrow, Path(TypePath { qself: None, path: Path { leading_colon: None, segments: [PathSegment { ident: Ident { ident: "U256", span: # 0 bytes(630..634) }, arguments: None }] } })) }, default : None, semi_token: Some(Semi) }
fn handle_call_func(func: syn::TraitItemMethod) -> proc_macro2::TokenStream {
    // func_hash_num 函数选择器(https://solidity-cn.readthedocs.io/zh/develop/abi-spec.html#function-selector)
    debug!("func: {:#?}", func.sig);
    // let func = func.clone();
    let func_ident = &func.sig.ident;
    let func_hash_num = gen_func_hash(&func.sig);
    let func_input_types = parse_func_input(func.sig.clone()).1;
    let func_output_types = parse_func_output(func.sig.clone());
    if func_output_types.is_empty(){
        quote! {
            #func_hash_num => {
                let mut stream = wasm2ct::types::Stream::new(method_payload);
                inner.#func_ident(
                    #(stream.pop::<#func_input_types>().expect("argument decoding failed")),*
                );
                Vec::new()
            }
        }
    }else{
        let output_len = syn::Lit::Int(syn::LitInt::new(&(func_output_types.len() as u64).to_string(),Span::call_site()));
        quote! {
            #func_hash_num => {
                let mut stream = wasm2ct::types::Stream::new(method_payload);
                let ret = inner.#func_ident(
                    #(stream.pop::<#func_input_types>().expect("argument decoding failed")),*
                );
                let mut sink = wasm2ct::types::Sink::new(#output_len);
                sink.push(ret);
                sink.finalize_panicking()
            }
        }
    }
}

// 生成constructor,event外的abi
pub fn gen_func_abi(func: syn::TraitItemMethod) -> FunctionAbi {
    // func_hash_num 函数选择器(https://solidity-cn.readthedocs.io/zh/develop/abi-spec.html#function-selector)
    debug!("func: {:#?}", func.sig);
    // let func = func.clone();
    let func_ident = &func.sig.ident;
    let mut input_args = Vec::new();
    let mut output_args = Vec::new();
    let func_output_types = parse_func_output(func.sig.clone());
    for fn_arg in func.sig.clone().inputs.iter(){
        match fn_arg{
            syn::FnArg::Typed(pat_type)=>{
                let input_pat = pat_type.pat.clone();
                let mut ret = String::new();
                push_type(&mut ret,&pat_type.ty);
                let argument = Argument{
                    argname: quote!{#input_pat}.to_string(),
                    argtype: ret,
                };
                input_args.push(argument)
            },
            _=>debug!("panic: {:?}",fn_arg)
        }
    };
    for (idx,output_type) in func_output_types.iter().enumerate(){
        let mut ret = String::new();
        push_type(&mut ret,output_type);
        let argument = Argument{
            argname: format!("arg{}",idx),
            argtype: ret,
        };
        output_args.push(argument)               
    }
    FunctionAbi{
        name:func_ident.to_string(),
        inputs:input_args,
        outputs:output_args,
    }

}





// 生成constructor abi
pub fn gen_constructor_abi(func: syn::TraitItemMethod) -> ConstructorAbi {
    // func_hash_num 函数选择器(https://solidity-cn.readthedocs.io/zh/develop/abi-spec.html#function-selector)
    debug!("func: {:#?}", func.sig);
    // let func = func.clone();
    let mut input_args = Vec::new();
    for fn_arg in func.sig.clone().inputs.iter(){
        match fn_arg{
            syn::FnArg::Typed(pat_type)=>{
                let input_pat = pat_type.pat.clone();
                let mut ret = String::new();
                push_type(&mut ret,&pat_type.ty);
                let argument = Argument{
                    argname: quote!{#input_pat}.to_string(),
                    argtype: ret,
                };
                input_args.push(argument)
            },
            _=>debug!("panic: {:?}",fn_arg)
        }
    };
    ConstructorAbi{
        inputs:input_args,
    }

}

// 生成event abi
pub fn gen_event_abi(func: syn::TraitItemMethod) -> EventAbi {

    let func_ident = &func.sig.ident;
    let mut input_args = Vec::new();
    for fn_arg in func.sig.clone().inputs.iter(){
        match fn_arg{
            syn::FnArg::Typed(pat_type)=>{
                let input_pat = pat_type.pat.clone();
                let mut argtype = String::new();
                push_type(&mut argtype,&pat_type.ty);
                let argname= quote!{#input_pat}.to_string();
                let indexed;
                if argname.starts_with("indexed"){
                    indexed = true;
                }else{
                    indexed = false;
                }
                let argument = EventInput{
                    argname,
                    argtype,
                    indexed
                };
                input_args.push(argument)
            },
            _=>debug!("panic: {:?}",fn_arg)
        }
    };
    EventAbi{
        name:func_ident.to_string(),
        inputs:input_args,
    }

}




fn handle_deploy_func(func: syn::TraitItemMethod) -> proc_macro2::TokenStream {
    let func_input_types = parse_func_input(func.sig).1;
    quote! {
            let mut stream = wasm2ct::types::Stream::new(payload);
            self.inner.constructor(
                #(stream.pop::<#func_input_types>().expect("argument decoding failed")),*
            );
		}
}




//
fn parse_func_input(sig:syn::Signature) ->(Vec<Box<syn::Pat>>,Vec<Box<syn::Type>>){
    let mut func_input_types:Vec<Box<syn::Type>> = Vec::new();
    let mut func_input_pats:Vec<Box<syn::Pat>> = Vec::new();
    for fn_arg in sig.clone().inputs.iter(){
        match fn_arg{
            syn::FnArg::Typed(pat_type)=>{
                let input_pat = pat_type.pat.clone();
                let input_type = pat_type.ty.clone();
                func_input_pats.push(input_pat);
                func_input_types.push(input_type);
            },
            _=>debug!("panic: {:?}",fn_arg)
        }
    };
    (func_input_pats,func_input_types)
}
//
fn parse_func_output(sig:syn::Signature) ->Vec<Box<syn::Type>>{
    let func_output_types:Vec<Box<syn::Type>> = match sig.output{
        syn::ReturnType::Default => Vec::new(),
        syn::ReturnType::Type(_,ty) =>{
            match *ty{
                syn::Type::Tuple(type_tuple)=>{
                    type_tuple.elems.into_iter().map(|tar_type|Box::new(tar_type)).collect()
                },
                ty => vec![Box::new(ty)],
            }
        }
    };
    func_output_types
}
//
// // 生成event函数体
fn split_pat(sig: &syn::Signature) ->(Vec<&Box<syn::Pat>>,Vec<&Box<syn::Pat>>){
    // println!("split_pat start --------------");
    let mut index_pats:Vec<&Box<syn::Pat>> = vec![];
    let mut other_pats:Vec<&Box<syn::Pat>> = vec![];
    for pat in sig.inputs.iter() {
        match pat{
            syn::FnArg::Typed(pat_type) =>{
                if (quote! {#pat_type.pat}.to_string()).starts_with("index_"){
                    index_pats.push(&pat_type.pat)
                }else {
                    other_pats.push(&pat_type.pat)
                }
            },
            _ => {}
        }
    }
    debug!("index_pats {:?} \n other_pats {:?}",index_pats,other_pats);
    (index_pats,other_pats)

}
//
//
//
fn gen_event_content(sig:&syn::Signature) ->proc_macro2::TokenStream{
    // derive_print!("gen_event_content start --------------");
    // println!("gen_event_content start --------------");
    let topics =  gen_topic(sig);
    let other_pats = split_pat(sig).1;
    let other_pats_len = syn::Lit::Int(syn::LitInt::new(&(other_pats.len() as u64).to_string(),Span::call_site()));
    // println!("gen_event_content end --------------");
    quote! {
        #topics
        let mut sink = ::wasm2ct::types::Sink::new(#other_pats_len);
        #(sink.push(#other_pats));*;
        let payload = sink.finalize_panicking();

        ::wasm_mid::log(topics, &payload);
    }
}
//
// // 生成topic
// // canonical: "Transfer(address,address,uint256)"
// // let topics = &[
// // [#(#hash_bytes),*].into(),
// // #(::wasm2ct::types::Convert2Log::convert2log(&#indexed_pats)),*
// // ];
// // let mut sink = ::wasm2ct::types::Sink::new(#data_pats_count_lit);
// // #(sink.push(#data_pats));*;
// // let payload = sink.finalize_panicking();
// //
// // ::wasm_mid::log(topics, &payload);
fn gen_topic(sig: &syn::Signature)->proc_macro2::TokenStream{
    // println!("gen_topic start {:?}--------------\n",sig);
    let func_types = get_func_types(sig);
    let keccak_hash = keccak(func_types.as_bytes());
    let hash_bytes = keccak_hash.as_ref().iter().map(|b| {
        syn::Lit::Int(syn::LitInt::new(&(*b as u64).to_string(), Span::call_site() ))
    });
    let index_pats = split_pat(sig).0;
    // println!("gen_topic end --------------");
    quote! {
            let topics = &[
                [#(#hash_bytes),*].into(),
                #(::wasm2ct::types::Convert2Log::convert2log(&#index_pats)),*
            ];
    }

}
//
// // 获取函数名称与参数类型例如:Transfer(address,address,uint256)
fn get_func_types(sig: &syn::Signature)->String{

    let mut ret = String::new();
    ret.push_str(&sig.ident.to_string());
    ret.push('(');
    // let inputs_len = sig.inputs.len();
    for item in &sig.inputs{
        match item{
            syn::FnArg::Typed(item) => {
                push_type(&mut ret,&item.ty);
                ret.push(',')
            },
            _ => {}
        }
    }
    if ret.chars().last().unwrap() == ','{
        ret.pop();
    }
    ret.push(')');
    // println!("funcname {}",ret);
    ret
}
//
//
pub fn gen_func_hash(sig: &syn::Signature)->u32{
    let func_types = get_func_types(sig);
    let keccak_hash = keccak(func_types.as_bytes());
    let tar =BigEndian::read_u32(&keccak_hash.as_ref()[0..4]);
    // println!("func hash {}",tar);
    tar
}
//
//
fn keccak(bytes: &[u8])->H256{
    let mut keccak = Keccak::v256();
    let mut res = H256::zero();
    keccak.update(bytes);
    keccak.finalize(res.as_mut());
    res
}
//
//
pub fn parse_event_trait(sig: syn::Signature)->proc_macro2::TokenStream{
    // println!("parse_event_trait {:?}",sig);
    assert_ne!(sig.ident, "constructor", "constructor 不能为 event");
    let event_name = sig.clone().ident;
    let event_args = sig.inputs.iter().filter_map(|arg|match arg{
        syn::FnArg::Typed(pat_type) => {
            let pat = &pat_type.pat;
            let ty = &pat_type.ty;
            Some(quote!{#pat:#ty})
        }
        _ => None,
    });
    // println!("parse_event_trait 272");
    let event_content = gen_event_content(&sig);

    match sig.output{
        syn::ReturnType::Type(_,ref ret) =>{
            quote! {
                fn #event_name(&mut self, #(#event_args),*) -> #ret{
                    #event_content
                }
            }
        },
        syn::ReturnType::Default =>{
            quote!{
                fn #event_name(&mut self,#(#event_args),*){
                    #event_content
                }
            }
        }
    }
}
//
fn push_type(ret:&mut String,pat_type: &Box<syn::Type>){
    match &**pat_type{
        syn::Type::Path(type_path) if type_path.qself.is_none() =>{
            if *&type_path.path.segments.len() != 1{
                panic!("type_path.path.segments len {}",type_path.path.segments.len())
            }
            // ret.push_str(&type_path.path.segments[0].ident.to_string())
            add_type(ret,type_path);
        },
        syn::Type::Array(type_array)=>{
            if let syn::Type::Path(type_path) = &*type_array.elem{
                if "u8" == type_path.path.segments.last().unwrap().ident.to_string(){
                    ret.push_str("bytes");
                    push_syn_expr(ret,&type_array.len);
                    return;
                }
            }
            panic!("不支持不定长数组")
        }
        _ => {}
    }
}
//
//
fn push_syn_expr(ret: &mut String,syn_expr: &syn::Expr){
    match syn_expr{
        syn::Expr::Lit(syn::ExprLit{lit: syn::Lit::Int(lit_int), ..}) => {
            // debug!("{:?}",lit_int.base10_digits().parse::<u64>().unwrap());
            ret.push_str(&format!("{:?}", lit_int.base10_digits().parse::<u64>().unwrap()))
        }
        _ => panic!("Cannot use something other than integer literal in this constant expression"),
    }
}
//
//
//
//
fn add_type(ret: &mut String, type_path: &syn::TypePath){
    let path_seg = type_path.path.segments.last().unwrap().ident.to_string();
    match path_seg.as_str(){
        "u32" =>ret.push_str("uint32"),
        "i32" =>ret.push_str("int32"),
        "u64" =>ret.push_str("uint64"),
        "i64" =>ret.push_str("int64"),
        "U256" =>ret.push_str("uint256"),
        "H256" =>ret.push_str("uint256"),
        "Address" =>ret.push_str("address"),
        "String" =>ret.push_str("string"),
        "bool" =>ret.push_str("bool"),
        "Vec" =>{
            // match &seg.segments[0].arguments{
            //     syn::PathArguments::AngleBracketed(args) =>{
            //         let type = args.args.last().unwrap();
            //         if let syn::GenericArgument::Type(syn::Type::Path(type_path)) = type {
            //             if type_path.qself.is_none() && type_path.path.segments.last().unwrap().ident == "u8"{
            //                 "bytes"
            //             }
            //
            //         }
            //     },
            //     _=>panic!()
            // }
            push_vec(ret, type_path);
            // panic!("vec is not support")
        },
        unexpected => panic!("不支持的数据类型: {}",unexpected),
    }
}
//
fn push_vec(ret: &mut String,type_path: &syn::TypePath){
    match &type_path.path.segments.last().unwrap().arguments{
        syn::PathArguments::AngleBracketed(gen_args) => {
            let last_type = gen_args.args.last().unwrap();
            if let syn::GenericArgument::Type(syn::Type::Path(type_path)) = last_type {
                return if type_path.qself.is_none()
                    && type_path.path.segments.last().unwrap().ident == "u8"
                {
                    ret.push_str("bytes");
                }
                else {
                    add_type(ret, type_path);
                    ret.push_str("[]");
                }
            }
            panic!("Unsupported generic arguments")
        },
        _ => panic!()
    }
}