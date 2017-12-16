extern crate which;

#[cfg(feature = "genproto")]
mod inner {
    use std::process::Command;
    use which::which;

    // NOTE: enable log with `cargo build -vv`
    pub fn gen() {
        println!("gen protoc");
        let grpc_rust_plugin_path = which("grpc_rust_plugin").expect("fail get extension path")
            .to_str().expect("fail path string").to_string();

        let output = Command::new("protoc")
            .arg("--rust_out=./src/")
            .arg("--grpc_out=./src/")
            .arg(format!("--plugin=protoc-gen-grpc={}", grpc_rust_plugin_path).as_str())
            .arg("proto/zothcore.proto")
            .output().expect("fail command");
        println!("{}", output.status);
        println!("{}", String::from_utf8_lossy(&output.stdout));
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }
}

#[cfg(not(feature = "genproto"))]
mod inner {
    pub fn gen() {}
}

fn main() {
    inner::gen();
}
