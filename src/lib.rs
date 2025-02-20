pub mod server;
mod protoc;
mod protobuf;
mod message_builder;
mod message_decoder;
mod utils;
mod matching;

pub mod built_info {
  include!(concat!(env!("OUT_DIR"), "/built.rs"));
}
