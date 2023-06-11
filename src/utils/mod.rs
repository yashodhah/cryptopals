extern crate hex;
extern crate base64;

pub mod base_utils {
    use hex::FromHex;
    use base64::{Engine as _, engine::general_purpose};

    pub fn hex_to_base64(hex_string: &str) -> String {
        let bytes = Vec::from_hex(hex_string).expect("Invalid hex string");
        let encoded: String = general_purpose::STANDARD_NO_PAD.encode(bytes);

        encoded
    }

    pub fn xor(v1: &Vec<u8>, v2: &Vec<u8>) -> Vec<u8> {
        if v1.len() != v2.len() {
            panic!("Buffer lengths are unequal");
        }

        let v3: Vec<u8> = v1
            .iter()
            .zip(v2.iter())
            .map(|(&x1, &x2)| x1 ^ x2)
            .collect();

        return v3;
    }
}