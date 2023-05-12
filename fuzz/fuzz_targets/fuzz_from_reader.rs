use honggfuzz::fuzz;

use ciborium::{de::from_reader, value::Value};

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            let _ = from_reader::<Value, _>(&data[..]);
        });
    }
}