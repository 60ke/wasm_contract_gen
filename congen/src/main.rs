use std::{fs, env};
use std::path::PathBuf;


// ./congen /Users/k/Desktop/congen_test contract_test
fn main() {
	let args = env::args().collect::<Vec<_>>();
	let (contract_path, contract_name) = match args.len() {
		3 => (&args[1], &args[2]),
		_ => {
			println!("Usage: {} gen <project_path> <contract_name>", args[0]);
			return;
		},
	};
    println!("{} {}",contract_path,contract_name);

	let toml = format!(r#"
[package]
name = "$contract_name"
version = "0.1.0"
authors = ["k <worileqing@163.com>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
wasm_std = {{path = "../../wasm_std"}}
wasm_mid = {{path = "../../wasm_mid"}}
wasm2ct = {{path = "../../wasm2ct"}}
wasm2ct_derive = {{path = "../../wasm2ct/wasm2ct_derive"}}
lazy_static = {{ version = "1.2.0", features = ["spin_no_std"] }}
wee_alloc = "0.4.5"

[lib]
crate-type = ["cdylib"]

[features]
std = ["wasm2ct/std", "wasm_mid/std"]

[profile.release]
panic = "abort"
lto = true
opt-level = "z"
    "#);

	let target_toml = toml.replace("$contract_name", contract_name);

	let crate_dir_path = PathBuf::from(contract_path);
 

	fs::create_dir_all(&crate_dir_path).expect(&format!("failed to create \"{}\" directory", crate_dir_path.to_string_lossy()));

    let mut source_path = crate_dir_path.clone();
    source_path.push("src");
	fs::create_dir_all(&source_path).expect(&format!("failed to create \"{}\" directory", source_path.to_string_lossy()));
    source_path.push("lib.rs");
    fs::File::create(source_path.clone()).expect(&format!("failed to create \"{}\" file", source_path.to_string_lossy()));       
	
    let mut toml_path = crate_dir_path.clone();
    toml_path.push("Cargo.toml");
    let mut f = fs::File::create(toml_path.clone()).expect(&format!("failed to create \"{}\" file", toml_path.to_string_lossy()));
    ::std::io::Write::write_all(&mut f, &target_toml.as_bytes()[..])
        .expect("Failed to write toml");
	

    let bash_file = format!(r#"
#!/bin/bash
cargo +nightly build --release --target wasm32-unknown-unknown
# wasm-build --target=wasm32-unknown-unknown ./target pwasm_tutorial_contract  
    "#);

    let mut bash_path = crate_dir_path.clone();
    bash_path.push("build.sh");
    let mut f = fs::File::create(bash_path.clone()).expect(&format!("failed to create \"{}\" file", bash_path.to_string_lossy()));
    ::std::io::Write::write_all(&mut f, &bash_file.as_bytes()[..])
        .expect("Failed to write build.sh");    
}
