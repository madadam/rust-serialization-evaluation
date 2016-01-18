#![feature(test, custom_derive, plugin)]
#![plugin(serde_macros)]

#![allow(dead_code)]

#[cfg(feature = "use_rustc_serialize")] extern crate rustc_serialize;
#[cfg(feature = "use_serde")] extern crate serde;

#[cfg(feature = "use_cbor")] extern crate cbor;
#[cfg(feature = "use_bincode")] extern crate bincode;

extern crate test;

#[cfg_attr(feature = "use_rustc_serialize", derive(RustcDecodable, RustcEncodable))]
#[cfg_attr(feature = "use_serde", derive(Deserialize, Serialize))]
pub struct Person {
  id:    u64,
  name:  String,
  email: String
}

#[cfg_attr(feature = "use_rustc_serialize", derive(RustcDecodable, RustcEncodable))]
#[cfg_attr(feature = "use_serde", derive(Deserialize, Serialize))]
pub struct Document {
  id:      u64,
  name:    String,
  authors: Vec<Person>
}

#[cfg(test)]
fn make_sample_data() -> Document {
  let alice = Person {
    id: 1,
    name: "Alice".to_owned(),
    email: "alice@example.com".to_owned()
  };

  let bob = Person {
    id: 2,
    name: "Bob".to_owned(),
    email: "bob@example.com".to_owned()
  };

  Document {
    id: 829472904,
    name: "stuff.txt".to_owned(),
    authors: vec![alice, bob]
  }
}

#[cfg(all(feature = "use_rustc_serialize", feature = "use_cbor"))]
mod code {
  use rustc_serialize::{Decodable, Encodable};
  use cbor::{Decoder, Encoder};

  pub fn encode<T: Encodable>(v: T) -> Vec<u8> {
    let mut encoder = Encoder::from_memory();
    encoder.encode(&[v]).unwrap();
    encoder.as_bytes().to_vec()
  }

  pub fn decode<T: Decodable>(bytes: &[u8]) -> T {
    let mut decoder = Decoder::from_bytes(bytes);
    decoder.decode().next().unwrap().unwrap()
  }
}

#[cfg(all(feature = "use_serde", feature = "use_bincode"))]
mod code {
  use serde::{Deserialize, Serialize};
  use bincode::SizeLimit;
  use bincode::serde;

  pub fn encode<T: Serialize>(v: T) -> Vec<u8> {
    serde::serialize(&v, SizeLimit::Infinite).unwrap()
  }

  pub fn decode<T: Deserialize>(bytes: &[u8]) -> T {
    serde::deserialize(bytes).unwrap()
  }
}

#[cfg(all(feature = "use_rustc_serialize", feature = "use_bincode"))]
mod code {
  use rustc_serialize::{Decodable, Encodable};
  use bincode::SizeLimit;
  use bincode::rustc_serialize;

  pub fn encode<T: Encodable>(v: T) -> Vec<u8> {
    rustc_serialize::encode(&v, SizeLimit::Infinite).unwrap()
  }

  pub fn decode<T: Decodable>(bytes: &[u8]) -> T {
    rustc_serialize::decode(bytes).unwrap()
  }
}

#[cfg(test)]
mod tests {
  use super::{code, make_sample_data, Document};
  use test::Bencher;

  #[bench]
  fn bench_serialize(b: &mut Bencher) {
    let doc = make_sample_data();

    b.iter(|| {
      code::encode(&doc);
    })
  }

  #[bench]
  fn bench_deserialize(b: &mut Bencher) {
    let doc = make_sample_data();
    let bytes = code::encode(&doc);

    b.iter(|| {
      code::decode::<Document>(&bytes);
    })
  }
}
