#![feature(test, custom_derive, plugin)]
#![plugin(serde_macros)]

#![allow(dead_code)]

#[cfg(feature = "use_cbor")] extern crate cbor;
#[cfg(feature = "use_cbor")] extern crate rustc_serialize;

#[cfg(feature = "use_bincode")] extern crate bincode;
#[cfg(feature = "use_bincode")] extern crate serde;

extern crate test;

#[cfg_attr(feature = "use_cbor", derive(RustcDecodable, RustcEncodable))]
#[cfg_attr(feature = "use_bincode", derive(Deserialize, Serialize))]
pub struct Person {
  id:    u64,
  name:  String,
  email: String
}

#[cfg_attr(feature = "use_cbor", derive(RustcDecodable, RustcEncodable))]
#[cfg_attr(feature = "use_bincode", derive(Deserialize, Serialize))]
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

#[cfg(test)]
mod tests {
  use super::{make_sample_data, Document};
  use test::Bencher;

  #[cfg(feature = "use_cbor")] use cbor::{Decoder, Encoder};
  #[cfg(feature = "use_cbor")] use rustc_serialize::{Decodable, Encodable};

  #[cfg(feature = "use_cbor")]
  fn encode<T: Encodable>(v: T) -> Vec<u8> {
    let mut encoder = Encoder::from_memory();
    encoder.encode(&[v]).unwrap();
    encoder.as_bytes().to_vec()
  }

  #[cfg(feature = "use_cbor")]
  fn decode<T: Decodable>(bytes: &[u8]) -> T {
    let mut decoder = Decoder::from_bytes(bytes);
    decoder.decode().next().unwrap().unwrap()
  }

  #[cfg(feature = "use_bincode")] use bincode::serde;
  #[cfg(feature = "use_bincode")] use bincode::SizeLimit;
  #[cfg(feature = "use_bincode")] use serde::{Deserialize, Serialize};

  #[cfg(feature = "use_bincode")]
  fn encode<T: Serialize>(v: T) -> Vec<u8> {
    serde::serialize(&v, SizeLimit::Infinite).unwrap()
  }

  #[cfg(feature = "use_bincode")]
  fn decode<T: Deserialize>(bytes: &[u8]) -> T {
    serde::deserialize(bytes).unwrap()
  }

  #[bench]
  fn bench_serialize(b: &mut Bencher) {
    let doc = make_sample_data();

    b.iter(|| {
      encode(&doc);
    })
  }

  #[bench]
  fn bench_deserialize(b: &mut Bencher) {
    let doc = make_sample_data();
    let bytes = encode(&doc);

    b.iter(|| {
      decode::<Document>(&bytes);
    })
  }
}

