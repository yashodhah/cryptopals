mod utils;

fn main() {
    // test_base64();
    // test_xor();

    let str1 = "test";
    println!("{}",hex::encode(str1));

}

fn test_base64() {
    let hex_string = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let base64_string = utils::base_utils::hex_to_base64(hex_string);

    assert_eq!(base64_string, "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
}

fn test_xor() {
    let str1 = "1c0111001f010100061a024b53535009181c";
    let str2 = "686974207468652062756c6c277320657965";

    let bytes1 = hex::decode(str1).expect("Failed to decode str1");
    let bytes2 = hex::decode(str2).expect("Failed to decode str2");

    let xor_vec = utils::base_utils::xor(&bytes1, &bytes2);
    let xor_hex = hex::encode(xor_vec);

    assert_eq!(xor_hex, "746865206b696420646f6e277420706c6179");
}


