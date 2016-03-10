#![feature(test, custom_derive, plugin)]
#![plugin(serde_macros)]

#![allow(dead_code)]

extern crate bincode;
extern crate cbor;
extern crate rand;
extern crate rmp_serialize;
extern crate rustc_serialize;
extern crate serde;
extern crate serde_cbor;
extern crate test;

mod rustc_serialize_traits;
mod serde_traits;
mod types;

mod code {
  pub mod rustc_serialize_cbor {
    use cbor::{Decoder, Encoder};
    use types::Document;

    pub fn encode(d: &Document) -> Vec<u8> {
      let mut encoder = Encoder::from_memory();
      encoder.encode(&[d]).unwrap();
      encoder.as_bytes().to_vec()
    }

    pub fn decode(bytes: &[u8]) -> Document {
      let mut decoder = Decoder::from_bytes(bytes);
      decoder.decode().next().unwrap().unwrap()
    }
  }

  pub mod serde_cbor {
    use serde_cbor::{de, ser};
    use types::Document;

    pub fn encode(d: &Document) -> Vec<u8> {
      ser::to_vec(d).unwrap()
    }

    pub fn decode(bytes: &[u8]) -> Document {
      de::from_slice(bytes).unwrap()
    }
  }

  pub mod serde_bincode {
    use bincode::SizeLimit;
    use bincode::serde;
    use types::Document;

    pub fn encode(d: &Document) -> Vec<u8> {
      serde::serialize(d, SizeLimit::Infinite).unwrap()
    }

    pub fn decode(bytes: &[u8]) -> Document {
      serde::deserialize(bytes).unwrap()
    }
  }

  pub mod rustc_serialize_bincode {
    use bincode::SizeLimit;
    use bincode::rustc_serialize;
    use types::Document;

    pub fn encode(d: &Document) -> Vec<u8> {
      rustc_serialize::encode(d, SizeLimit::Infinite).unwrap()
    }

    pub fn decode(bytes: &[u8]) -> Document {
      rustc_serialize::decode(bytes).unwrap()
    }
  }

  pub mod rustc_serialize_rmp {
    use rmp_serialize::{Decoder, Encoder};
    use rustc_serialize::{Decodable, Encodable};
    use std::io::Cursor;
    use types::Document;

    pub fn encode(d: &Document) -> Vec<u8> {
        let mut buffer = Vec::new();

        {
            let mut encoder = Encoder::new(&mut buffer);
            d.encode(&mut encoder).unwrap();
        }

        buffer
    }

    pub fn decode(bytes: &[u8]) -> Document {
        let mut cursor = Cursor::new(bytes);
        let mut decoder = Decoder::new(&mut cursor);
        Decodable::decode(&mut decoder).unwrap()
    }
  }
}

#[cfg(test)]
mod tests {
    use super::code;
    use types::{Document, make_sample_data};
    use test::Bencher;

    fn compute_size<E, D>(encode: E, decode: D) -> (usize, usize)
        where E: Fn(&Document) -> Vec<u8>,
              D: Fn(&[u8]) -> Document
    {
        let small = make_sample_data(0);
        let big = make_sample_data(1024 * 1024);

        let small_encoded = encode(&small);
        let big_encoded = encode(&big);

        assert_eq!(small, decode(&small_encoded));
        assert_eq!(big, decode(&big_encoded));

        (small_encoded.len(), big_encoded.len())
    }

    fn print_size(sizes: (usize, usize)) {
        println!("    small: {}", sizes.0);
        println!("    big:   {}", sizes.1);
    }

    fn bench_serialize<E>(b: &mut Bencher, size: usize, encode: E)
        where E: Fn(&Document) -> Vec<u8>
    {
        let doc = make_sample_data(size);

        b.iter(|| {
            encode(&doc);
        })
    }

    fn bench_deserialize<E, D>(b: &mut Bencher, size: usize, encode: E, decode: D)
        where E: Fn(&Document) -> Vec<u8>,
              D: Fn(&[u8]) -> Document
    {
        let doc = make_sample_data(size);
        let bytes = encode(&doc);

        b.iter(|| {
            decode(&bytes);
        })
    }

    #[test]
    fn size() {
        println!("");
        println!("rustc_serialize + CBOR");
        print_size(compute_size(code::rustc_serialize_cbor::encode,
                                code::rustc_serialize_cbor::decode));

        println!("rustc_serialize + bincode");
        print_size(compute_size(code::rustc_serialize_bincode::encode,
                                code::rustc_serialize_bincode::decode));

        // println!("serde + CBOR");
        // print_size(compute_size(code::serde_cbor::encode,
        //                         code::serde_cbor::decode));

        println!("serde + bincode");
        print_size(compute_size(code::serde_bincode::encode,
                                code::serde_bincode::decode));

        println!("rustc_serialize + rmp");
        print_size(compute_size(code::rustc_serialize_rmp::encode,
                                code::rustc_serialize_rmp::decode));
    }

    #[bench]
    fn bench_serialize_small_rustc_serialize_cbor(b: &mut Bencher) {
        bench_serialize(b, 0, code::rustc_serialize_cbor::encode);
    }

    #[bench]
    fn bench_serialize_small_rustc_serialize_bincode(b: &mut Bencher) {
        bench_serialize(b, 0, code::rustc_serialize_bincode::encode);
    }

    #[bench]
    fn bench_serialize_small_serde_cbor(b: &mut Bencher) {
        bench_serialize(b, 0, code::serde_cbor::encode);
    }

    #[bench]
    fn bench_serialize_small_serde_bincode(b: &mut Bencher) {
        bench_serialize(b, 0, code::serde_bincode::encode);
    }

    #[bench]
    fn bench_serialize_small_rustc_serialize_rmp(b: &mut Bencher) {
        bench_serialize(b, 0, code::rustc_serialize_rmp::encode);
    }

    #[bench]
    fn bench_serialize_big_rustc_serialize_cbor(b: &mut Bencher) {
        bench_serialize(b, 1024 * 1024, code::rustc_serialize_cbor::encode);
    }

    #[bench]
    fn bench_serialize_big_rustc_serialize_bincode(b: &mut Bencher) {
        bench_serialize(b, 1024 * 1024, code::rustc_serialize_bincode::encode);
    }

    #[bench]
    fn bench_serialize_big_serde_cbor(b: &mut Bencher) {
        bench_serialize(b, 1024 * 1024, code::serde_cbor::encode);
    }

    #[bench]
    fn bench_serialize_big_serde_bincode(b: &mut Bencher) {
        bench_serialize(b, 1024 * 1024, code::serde_bincode::encode);
    }

    #[bench]
    fn bench_serialize_big_rustc_serialize_rmp(b: &mut Bencher) {
        bench_serialize(b, 1024 * 1024, code::rustc_serialize_rmp::encode);
    }



    #[bench]
    fn bench_deserialize_small_rustc_serialize_cbor(b: &mut Bencher) {
        bench_deserialize(b, 0, code::rustc_serialize_cbor::encode,
                                code::rustc_serialize_cbor::decode);
    }

    #[bench]
    fn bench_deserialize_small_rustc_serialize_bincode(b: &mut Bencher) {
        bench_deserialize(b, 0, code::rustc_serialize_bincode::encode,
                                code::rustc_serialize_bincode::decode);
    }

    #[bench]
    fn bench_deserialize_small_serde_cbor(b: &mut Bencher) {
        bench_deserialize(b, 0, code::serde_cbor::encode,
                                code::serde_cbor::decode);
    }

    #[bench]
    fn bench_deserialize_small_serde_bincode(b: &mut Bencher) {
        bench_deserialize(b, 0, code::serde_bincode::encode,
                                code::serde_bincode::decode);
    }

    #[bench]
    fn bench_deserialize_small_rustc_serialize_rmp(b: &mut Bencher) {
        bench_deserialize(b, 0, code::rustc_serialize_rmp::encode,
                                code::rustc_serialize_rmp::decode);
    }

    #[bench]
    fn bench_deserialize_big_rustc_serialize_cbor(b: &mut Bencher) {
        bench_deserialize(b, 1024 * 1024, code::rustc_serialize_cbor::encode,
                                          code::rustc_serialize_cbor::decode);
    }

    #[bench]
    fn bench_deserialize_big_rustc_serialize_bincode(b: &mut Bencher) {
        bench_deserialize(b, 1024 * 1024, code::rustc_serialize_bincode::encode,
                                          code::rustc_serialize_bincode::decode);
    }

    #[bench]
    #[ignore]
    fn bench_deserialize_big_serde_cbor(b: &mut Bencher) {
        bench_deserialize(b, 1024 * 1024, code::serde_cbor::encode,
                                          code::serde_cbor::decode);
    }

    #[bench]
    fn bench_deserialize_big_serde_bincode(b: &mut Bencher) {
        bench_deserialize(b, 1024 * 1024, code::serde_bincode::encode,
                                          code::serde_bincode::decode);
    }

    #[bench]
    fn bench_deserialize_big_rustc_serialize_rmp(b: &mut Bencher) {
        bench_deserialize(b, 1024 * 1024, code::rustc_serialize_rmp::encode,
                                          code::rustc_serialize_rmp::decode);
    }
}
