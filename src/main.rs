mod utils;

fn main() {
    let hex_string = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let base64_string = utils::base_utils::hex_to_base64(hex_string);

    println!("Base64: {}", base64_string);
}
