use std::process::Command;
use std::env;
use std::path::{Path,PathBuf};
use cmake::Config;
extern crate bindgen;

fn build_using_cmake(){
	let build_dst = Config::new("kfr").generator("Unix Makefiles")
				.define("ENABLE_CAPI_BUILD","ON")
				.define("CMAKE_BUILD_TYPE","Release")
				.define("CMAKE_CXX_COMPILER","clang++")
				.build();
}

fn check_submodule_exists(){

	if !Path::new("kfr/.git").exists() {
        let _ = Command::new("git")
            .args(&["submodule", "update", "--init", "libgit2"])
            .status();
    }
}

fn main(){
	let project_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
	let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());
	let target = env::var("TARGET").unwrap();
	let header_path = dst.join("include/kfr/capi.h").into_os_string()
						.into_string().unwrap();
	let library_path = dst.join("lib").into_os_string().into_string().unwrap();
	let DYLIB_NAME = "kfr_capi";

	// check whether the kfr module exists
	check_submodule_exists();

	// build the kfr library using cmake
	// as mentioned in here: https://github.com/kfrlib/kfr#building-kfr-c-api
	build_using_cmake();
	
	// search the path where for "libkfr_capi.dylib" where it is being created
	println!("cargo:rustc-link-search=native={}",library_path);
	//link kfr_capi dylib
	println!("cargo:rustc-link-lib=dylib={}",DYLIB_NAME);	

	// Auto generating bindings using bindgen crate.
	let bindings = bindgen::Builder::default()
	 				.header(header_path)
	 				.generate()
	 				.expect("Unable to generate bindings for kfr");

	//binding.rs file is create in src folder of the project.
	bindings.write_to_file(project_dir.join("src/bindings.rs"))
	 		.expect("Could not write bindings");

}