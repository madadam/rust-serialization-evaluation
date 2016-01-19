#![cfg(all(feature = "use_serde", feature = "explicit_traits"))]

use types::{Document, Person};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::{de, ser};

// Serialization ---------------------------------------------------------------

impl Serialize for Document {
  fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where S: Serializer
  {
    serializer.visit_struct("Document", DocumentSerializeVisitor {
      value: self,
      state: 0,
    })
  }
}

struct DocumentSerializeVisitor<'a> {
  value: &'a Document,
  state: u8,
}

impl<'a> ser::MapVisitor for DocumentSerializeVisitor<'a> {
  fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
    where S: Serializer
  {
    match self.state {
      0 => {
        self.state += 1;
        Ok(Some(try!(serializer.visit_struct_elt("id", &self.value.id))))
      }

      1 => {
        self.state += 1;
        Ok(Some(try!(serializer.visit_struct_elt("name", &self.value.name))))
      }

      2 => {
        self.state += 1;
        Ok(Some(try!(serializer.visit_struct_elt("authors", &self.value.authors))))
      }

      3 => {
        self.state += 1;
        Ok(Some(try!(serializer.visit_struct_elt("content", &self.value.content))))
      }

      _ => {
        Ok(None)
      }
    }
  }
}

impl Serialize for Person {
  fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where S: Serializer
  {
    serializer.visit_struct("Person", PersonSerializeVisitor {
      value: self,
      state: 0
    })
  }
}

struct PersonSerializeVisitor<'a> {
  value: &'a Person,
  state: u8,
}


impl<'a> ser::MapVisitor for PersonSerializeVisitor<'a> {
  fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
    where S: Serializer
  {
    match self.state {
      0 => {
        self.state += 1;
        Ok(Some(try!(serializer.visit_struct_elt("id", &self.value.id))))
      }

      1 => {
        self.state += 1;
        Ok(Some(try!(serializer.visit_struct_elt("name", &self.value.name))))
      }

      2 => {
        self.state += 1;
        Ok(Some(try!(serializer.visit_struct_elt("email", &self.value.email))))
      }

      _ => {
        Ok(None)
      }
    }
  }
}

// Deserialization -------------------------------------------------------------

enum DocumentField {
  Id, Name, Authors, Content
}

impl Deserialize for DocumentField {
  fn deserialize<D>(deserializer: &mut D) -> Result<DocumentField, D::Error>
    where D: Deserializer
  {
    deserializer.visit(DocumentFieldVisitor)
  }
}

struct DocumentFieldVisitor;

impl de::Visitor for DocumentFieldVisitor {
  type Value = DocumentField;

  fn visit_str<E>(&mut self, value: &str) -> Result<DocumentField, E>
    where E: de::Error
  {
    match value {
      "id"      => Ok(DocumentField::Id),
      "name"    => Ok(DocumentField::Name),
      "authors" => Ok(DocumentField::Authors),
      "content" => Ok(DocumentField::Content),
      _         => Err(de::Error::syntax("unexpected field")),
    }
  }
}

impl Deserialize for Document {
  fn deserialize<D>(deserializer: &mut D) -> Result<Document, D::Error>
    where D: Deserializer
  {
    static FIELDS: &'static [&'static str] = &["id", "name", "authors", "content"];
    deserializer.visit_struct("Document", FIELDS, DocumentDeserializeVisitor)
  }
}

struct DocumentDeserializeVisitor;

impl de::Visitor for DocumentDeserializeVisitor {
  type Value = Document;

  fn visit_map<V>(&mut self, mut visitor: V) -> Result<Document, V::Error>
    where V: de::MapVisitor
  {
    let mut document = Document {
      id: 0,
      name: String::new(),
      authors: Vec::new(),
      content: String::new()
    };

    loop {
      match try!(visitor.visit_key()) {
        Some(DocumentField::Id)      => { document.id      = try!(visitor.visit_value()); }
        Some(DocumentField::Name)    => { document.name    = try!(visitor.visit_value()); }
        Some(DocumentField::Authors) => { document.authors = try!(visitor.visit_value()); }
        Some(DocumentField::Content) => { document.content = try!(visitor.visit_value()); }
        None                         => break
      }
    }

    try!(visitor.end());

    Ok(document)
  }
}

enum PersonField {
  Id, Name, Email
}

impl Deserialize for PersonField {
  fn deserialize<D>(deserializer: &mut D) -> Result<PersonField, D::Error>
    where D: Deserializer
  {
    deserializer.visit(PersonFieldVisitor)
  }
}

struct PersonFieldVisitor;

impl de::Visitor for PersonFieldVisitor {
  type Value = PersonField;

  fn visit_str<E>(&mut self, value: &str) -> Result<PersonField, E>
    where E: de::Error
  {
    match value {
      "id"    => Ok(PersonField::Id),
      "name"  => Ok(PersonField::Name),
      "email" => Ok(PersonField::Email),
      _       => Err(de::Error::syntax("unexpected field")),
    }
  }
}

impl Deserialize for Person {
  fn deserialize<D>(deserializer: &mut D) -> Result<Person, D::Error>
    where D: Deserializer
  {
    static FIELDS: &'static [&'static str] = &["id", "name", "email"];
    deserializer.visit_struct("Person", FIELDS, PersonDeserializeVisitor)
  }
}

struct PersonDeserializeVisitor;

impl de::Visitor for PersonDeserializeVisitor {
  type Value = Person;

  fn visit_map<V>(&mut self, mut visitor: V) -> Result<Person, V::Error>
    where V: de::MapVisitor
  {
    let mut person = Person {
      id: 0,
      name: String::new(),
      email: String::new(),
    };

    loop {
      match try!(visitor.visit_key()) {
        Some(PersonField::Id)    => { person.id    = try!(visitor.visit_value()); }
        Some(PersonField::Name)  => { person.name  = try!(visitor.visit_value()); }
        Some(PersonField::Email) => { person.email = try!(visitor.visit_value()); }
        None                     => break
      }
    }

    try!(visitor.end());

    Ok(person)
  }
}
