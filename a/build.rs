fn main() {
    let protoc_path = protoc_bin_vendored::protoc_bin_path().unwrap();
    std::env::set_var("PROTOC",protoc_path);
    prost_build::Config::new()
        .prost_path("b::prost")
        .compile_protos(&["./a.proto"],&["./"]).unwrap();
}
