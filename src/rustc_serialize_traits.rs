#![cfg(all(feature = "use_rustc_serialize", feature = "explicit_traits"))]

use types::{Document, Person};
use rustc_serialize::{Decodable, Decoder, Encodable, Encoder};

impl Encodable for Document {
  fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
    s.emit_struct("Document", 4, |s| {
      try!(s.emit_struct_field("id",      0, |s| self.id.encode(s)));
      try!(s.emit_struct_field("name",    1, |s| self.name.encode(s)));
      try!(s.emit_struct_field("authors", 2, |s| self.authors.encode(s)));
      try!(s.emit_struct_field("content", 3, |s| self.content.encode(s)));

      Ok(())
    })
  }
}

impl Encodable for Person {
  fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
    s.emit_struct("Person", 3, |s| {
      try!(s.emit_struct_field("id",    0, |s| self.id.encode(s)));
      try!(s.emit_struct_field("name",  1, |s| self.name.encode(s)));
      try!(s.emit_struct_field("email", 2, |s| self.email.encode(s)));

      Ok(())
    })
  }
}

impl Decodable for Document {
  fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
    d.read_struct("Document", 4, |d| {
      let document = Document {
        id:      try!(d.read_struct_field("id",      0, |d| Decodable::decode(d))),
        name:    try!(d.read_struct_field("name",    1, |d| Decodable::decode(d))),
        authors: try!(d.read_struct_field("authors", 2, |d| Decodable::decode(d))),
        content: try!(d.read_struct_field("content", 3, |d| Decodable::decode(d))),
      };

      Ok(document)
    })
  }
}

impl Decodable for Person {
  fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
    d.read_struct("Person", 3, |d| {
      let person = Person {
        id:    try!(d.read_struct_field("id",    0, |d| Decodable::decode(d))),
        name:  try!(d.read_struct_field("name",  1, |d| Decodable::decode(d))),
        email: try!(d.read_struct_field("email", 2, |d| Decodable::decode(d))),
      };

      Ok(person)
    })
  }
}