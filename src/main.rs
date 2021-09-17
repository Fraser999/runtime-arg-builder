use std::{collections::BTreeMap, fs};

use serde::Serialize;

use casper_types::{
    bytesrepr::{self, ToBytes},
    CLTyped, CLValue,
};

#[derive(Debug, Serialize)]
struct RawBytes {
    #[serde(serialize_with = "hex::serialize")]
    raw_bytes: Vec<u8>,
}

impl RawBytes {
    fn new(value: impl ToBytes + CLTyped) -> Self {
        let cl_value = CLValue::from_t(value)
            .unwrap_or_else(|error| panic!("failed constructing CLValue: {:?}", error));
        let raw_bytes = bytesrepr::serialize(cl_value)
            .unwrap_or_else(|error| panic!("should serialize value: {:?}", error));
        RawBytes { raw_bytes }
    }
}

#[derive(Debug, Serialize)]
struct DeployArg {
    /// The runtime arg's name.
    name: String,
    /// The runtime arg's encoded value.
    ///
    /// This should be a `CLValue`, serialized via `bytesrepr`.  When the `DeployArg` is
    /// JSON-encoded, this `value` gets hex-encoded.
    value: RawBytes,
}

impl DeployArg {
    fn new(name: &str, value: impl ToBytes + CLTyped) -> Self {
        DeployArg {
            name: name.to_string(),
            value: RawBytes::new(value),
        }
    }
}

fn main() {
    // First runtime arg.
    let arg1 = DeployArg::new("amount", 999_u64);

    // Second runtime arg.
    let mut map = BTreeMap::new();
    map.insert(1, "A");
    map.insert(2, "B");
    let arg2 = DeployArg::new("my_map", map);

    // Create a `Vec` of the args and JSON-encode them.
    let args = vec![arg1, arg2];
    let output = serde_json::to_vec(&args).expect("should encode args");

    // Write to a file.
    let path = "runtime_args.json";
    fs::write(path, output).unwrap_or_else(|error| panic!("failed writing to {}: {}", path, error));

    println!("wrote args to {}", path);
}
