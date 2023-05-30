use bincode::{deserialize as from_bincode, serialize as to_bincode};
use serde_cbor::{from_slice as from_cbor, to_vec as to_cbor};
use serde_derive::{Deserialize, Serialize};
use serde_json::{from_str as from_json, to_string as to_json};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct City {
    name: String,
    population: usize,
    latitude: f64,
    longitude: f64,
}

fn main() {
    let wenzhou = City {
        name: "Wenzhou".to_string(),
        population: 6_642_592,
        latitude: 27.994267,
        longitude: 120.699364,
    };

    // Serialize
    let as_json = to_json(&wenzhou).unwrap();
    let as_cbor = to_cbor(&wenzhou).unwrap();
    let as_bincode = to_bincode(&wenzhou).unwrap();

    println!("json:\n{}\n", as_json);
    println!("cbor:\n{:?}\n", as_cbor);
    println!("bincode:\n{:?}\n", as_bincode);
    println!(
        "json (as UTF-8):\n{}\n",
        String::from_utf8_lossy(as_json.as_bytes())
    );
    println!("cbor (as UTF-8):\n{}\n", String::from_utf8_lossy(&as_cbor));
    println!(
        "bincode (as UTF-8):\n{}\n",
        String::from_utf8_lossy(&as_bincode)
    );

    // Deserialize
    assert_eq!(from_json::<City>(&as_json).unwrap(), wenzhou);
    assert_eq!(from_bincode::<City>(&as_bincode).unwrap(), wenzhou);
    assert_eq!(from_cbor::<City>(&as_cbor).unwrap(), wenzhou);

    println!("Hello, world!");
}
