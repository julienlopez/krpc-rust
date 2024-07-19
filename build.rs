fn main() {
    tonic_build::configure()
        .build_server(true)
        .out_dir("src")
        .compile(
            &["submodules/krpc/protobuf/krpc.proto"],
            &["submodules/krpc/protobuf/"],
        )
        .unwrap();
}
