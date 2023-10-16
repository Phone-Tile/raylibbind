use std::{path::PathBuf, env};

use bindgen::CargoCallbacks;






fn main() {

    let libdir_path = PathBuf::from("raylib/src")
        .canonicalize()
        .expect("cannot canonicalize path");

    let headers_path = libdir_path.join("raylib.h");
    let headers_path_str = headers_path.to_str().expect("Path is not valid string");


    println!("cargo:rustc-link-search={}",libdir_path.to_str().unwrap());
    println!("cargo:rustc-link-lib=raylib");

    let abi = vec!["arm","arm64","x86","x86_64"];



    // let command = std::process::Command::new("pwd")
    //     .current_dir("raylib/src")
    //     .output()
    //     .expect("oupsi");

    // let out = String::from_utf8(command.stdout).unwrap();
    // println!("cargo:warning={}",out);

    // println!("cargo:warning={}","ok ?");


    if !std::process::Command::new("make")
        .current_dir("raylib/src")
        .arg("clean")
        .output()
        .expect("could not spawn `make`")
        .status
        .success()
    {
        panic!("error in make");
    }

    for arch in abi {
        println!("cargo:warning={}","start");
        println!("cargo:warning={}",arch);
        if !std::process::Command::new("make")
            .current_dir("raylib/src")
            .arg("PLATFORM=PLATFORM_ANDROID")
            .arg("ANDROID_NDK=../../android/ndk")
            .arg(format!("ANDROID_ARCH={}",arch))
            .arg("ANDROID_API_VERSION=29")
            .output()
            .expect("could not spawn `make`")
            .status
            .success()
            {
                panic!("error in make {}",arch);
            }
        println!("cargo:warning={}","cp");
        if !std::process::Command::new("cp")
            .current_dir("raylib/src")
            .arg("libraylib.a")
            .arg(format!("../../lib/{}",arch))
            .output()
            .expect("could not spawn `cp`")
            .status
            .success()
            {
                panic!("error in cp");
            }
        println!("cargo:warning={}","clean");
        if !std::process::Command::new("make")
            .current_dir("raylib/src")
            .arg("clean")
            .output()
            .expect("could not spawn `make`")
            .status
            .success()
            {
                panic!("error in make");
            }
    }


    let bindings = bindgen::Builder::default()
        .header(headers_path_str)
        .parse_callbacks(Box::new(CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");


    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");

    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings !");

}
