use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // override the build location, in order to check in the changes to proto files
    let curr_out = env::var("OUT_DIR").unwrap();
    env::set_var("OUT_DIR", "src");
    // let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("protos/flight_fusion");
    let root = PathBuf::from("../").join("protos/flight_fusion");
    let proto_files = vec![root.join("flight_fusion/ipc/v1alpha1/flight.proto")];

    let descriptor_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("proto_descriptor.bin");

    tonic_build::configure()
        // protoc in unbuntu builder needs this option
        .protoc_arg("--experimental_allow_proto3_optional")
        .file_descriptor_set_path(&descriptor_path)
        // Generate prost structs
        .compile(&proto_files, &[root])?;

    let descriptor_set = std::fs::read(descriptor_path)?;
    pbjson_build::Builder::new()
        .register_descriptors(&descriptor_set)?
        .build(&[".flight_fusion.ipc.v1alpha1"])?;

    // Prost currently generates an empty file, this was fixed but then reverted
    // https://github.com/tokio-rs/prost/pull/639
    // let google_protobuf_rs = Path::new("src/sql/google.protobuf.rs");
    // if google_protobuf_rs.exists() && google_protobuf_rs.metadata().unwrap().len() == 0 {
    //     std::fs::remove_file(google_protobuf_rs).unwrap();
    // }

    // Build tauri app
    env::set_var("OUT_DIR", curr_out);
    tauri_build::build();

    // As the proto file is checked in, the build should not fail if the file is not found
    Ok(())
}
