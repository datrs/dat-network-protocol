// Uncomment method below for like local builds.
fn main() -> Result<(), Box<dyn std::error::Error>> {
  // protobuf_codegen_pure::run(protobuf_codegen_pure::Args {
  //   out_dir: "src",
  //   input: &[
  //     "protos/cancel.proto",
  //     "protos/data.proto",
  //     "protos/data.proto",
  //     "protos/feed.proto",
  //     "protos/handshake.proto",
  //     "protos/have.proto",
  //     "protos/info.proto",
  //     "protos/request.proto",
  //     "protos/unhave.proto",
  //     "protos/unwant.proto",
  //     "protos/want.proto",
  //   ],
  //   customize: protobuf_codegen_pure::Customize {
  //     ..Default::default()
  //   },
  //   includes: &["protos"],
  // })?;
  Ok(())
}
