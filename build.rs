fn main() {
	cc::Build::new()
		.cpp(true)
		.file("src/main.cpp")
		.cpp_link_stdlib("stdc++")
		.compile("bug_cpp");
	println!("cargo:rustc-link-search={}",std::env::var("OUT_DIR").unwrap());
	println!("cargo:rustc-link-lib=bug_cpp");
	println!("cargo:rustc-link-lib=stdc++");
	let bindings = bindgen::Builder::default()
		.header("src/main.cpp")
		.parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
		.allowlist_item("Bug.*")
		.allowlist_item(".*bug.*")
		.vtable_generation(true)
		.generate()
		.expect("Unable to generate bindings");
	
	// Write the bindings to the $OUT_DIR/bindings.rs file.
	let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
	bindings
		.write_to_file(out_path.join("bindings.rs"))
		.expect("Couldn't write bindings!");
}
