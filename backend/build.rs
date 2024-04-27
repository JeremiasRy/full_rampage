use std::env;
use std::fs::{read_to_string, File};
use std::io::{BufWriter, Write};
use std::path::Path;
use std::process::Command;

extern crate protoc_rust;

fn main() {
    let out_dir_env = env::var_os("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir_env);
    protoc_rust::Codegen::new()
        .out_dir(out_dir)
        .inputs(&["protos/messages.proto"])
        .include("protos")
        .run()
        .expect("Running protoc failed.");

    let path = out_dir.join("messages.rs");
    let code = read_to_string(&path).expect("Failed to read generated file");
    let mut writer = BufWriter::new(File::create(path).unwrap());
    for line in code.lines() {
        if !line.starts_with("//!") && !line.starts_with("#!") {
            writer.write_all(line.as_bytes()).unwrap();
            writer.write_all(&[b'\n']).unwrap();
        }
    }
    let _ = Command::new("cargo").args(["fix", "--lib", "-p", "backend"]).spawn();
}