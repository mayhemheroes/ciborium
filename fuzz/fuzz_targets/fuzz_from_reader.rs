use honggfuzz::fuzz;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            let _ = ciborium::de::from_reader::<ciborium::value::Value, _>(&data[..]);
        });
    }
}