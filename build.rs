use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_file = "./proto/user.proto";
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional")
        .build_client(true)
        .build_server(true)
        .file_descriptor_set_path(out_dir.join("grpc_descriptor.bin"))
        .out_dir("./src/grpc/proto")
        .compile_protos(&[proto_file], &["proto"])?;

    Ok(())
}
