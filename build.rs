extern crate protoc_rust;

fn main() {
  protoc_rust::run(protoc_rust::Args {
    out_dir: "src",
    input: &[
      "protos/cancel.proto",
      "protos/data.proto",
      "protos/data.proto",
      "protos/feed.proto",
      "protos/handshake.proto",
      "protos/have.proto",
      "protos/info.proto",
      "protos/request.proto",
      "protos/unhave.proto",
      "protos/unwant.proto",
      "protos/want.proto",
    ],
    includes: &["protos"],
  }).expect("protoc");
}
