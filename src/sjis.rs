use encoding_rs::SHIFT_JIS;

pub fn decode(binary: Vec<u8>) -> String {
    let (decoded, _, _) = SHIFT_JIS.decode(&binary);
    decoded.to_string()
}

pub fn encode(text: &str) -> Vec<u8> {
    let (encoded, _, _) = SHIFT_JIS.encode(&text);
    encoded.to_vec()
}

pub fn is_sjis(binary: &Vec<u8>) -> bool {
    let t = String::from_utf8(binary.to_vec());
    match t {
        Ok(_) => false,
        Err(_) => true,
    }
}

