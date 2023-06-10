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
}