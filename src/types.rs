#[derive(Debug, PartialEq)]
#[derive(RustcDecodable, RustcEncodable)]
#[derive(Deserialize, Serialize)]
pub struct Person {
    pub id: u64,
    pub name: String,
    pub email: String,
}

#[derive(Debug, PartialEq)]
#[derive(RustcDecodable, RustcEncodable)]
#[derive(Deserialize, Serialize)]
pub struct Document {
    pub id: u64,
    pub name: String,
    pub authors: Vec<Person>,

    #[cfg(feature = "binary_data")]
    pub content: Vec<u8>,

    #[cfg(not(feature = "binary_data"))]
    pub content: String,
}

#[cfg(test)]
pub fn make_sample_data(size: usize) -> Document {
    let alice = Person {
        id: 1,
        name: "Alice".to_owned(),
        email: "alice@example.com".to_owned(),
    };

    let bob = Person {
        id: 2,
        name: "Bob".to_owned(),
        email: "bob@example.com".to_owned(),
    };

    Document {
        id: 829472904,
        name: "stuff.txt".to_owned(),
        authors: vec![alice, bob],
        content: generate_data(size),
    }
}

#[cfg(test)]
#[cfg(feature = "binary_data")]
fn generate_data(size: usize) -> Vec<u8> {
    (0..256).cycle().map(|i| i as u8).take(size).collect()
}

#[cfg(test)]
#[cfg(not(feature = "binary_data"))]
fn generate_data(size: usize) -> String {
    use rand::{thread_rng, Rng};
    thread_rng().gen_ascii_chars().take(size).collect()
}
