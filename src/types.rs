#[derive(Debug, PartialEq)]
#[cfg_attr(all(feature = "use_rustc_serialize", not(feature = "explicit_traits")), derive(RustcDecodable, RustcEncodable))]
#[cfg_attr(all(feature = "use_serde",           not(feature = "explicit_traits")), derive(Deserialize, Serialize))]
pub struct Person {
  pub id:    u64,
  pub name:  String,
  pub email: String
}

#[derive(Debug, PartialEq)]
#[cfg_attr(all(feature = "use_rustc_serialize", not(feature = "explicit_traits")), derive(RustcDecodable, RustcEncodable))]
#[cfg_attr(all(feature = "use_serde",           not(feature = "explicit_traits")), derive(Deserialize, Serialize))]
pub struct Document {
  pub id:      u64,
  pub name:    String,
  pub authors: Vec<Person>,
  pub content: String,
}

#[cfg(test)]
pub fn make_sample_data(size: usize) -> Document {
  use rand::{thread_rng, Rng};

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
    authors: vec![alice, bob],
    content: thread_rng().gen_ascii_chars().take(size).collect(),
  }
}

