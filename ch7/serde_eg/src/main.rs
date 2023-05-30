use bincode::serialize as to_bincode;
use serde_cbor::to_vec as to_cbor;
use serde_derive::Serialize;
use serde_json::to_string as to_json;

#[derive(Serialize)]
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
    println!("Hello, world!");
}
