extern crate sha1;
extern crate rustc_serialize;

use rustc_serialize::base64::ToBase64;
use rustc_serialize::base64::STANDARD;

/// Generate sha1 rfc key as required by web socket protocol
pub fn generate_rfc_key(key: &String) -> String {

    let mut m = sha1::Sha1::new();

    // take submitted key, 
    m.update(key.as_bytes());
    // and rfc specified key
    m.update("258EAFA5-E914-47DA-95CA-C5AB0DC85B11".as_bytes());

    // hash it, and then encode it in base64
    m.digest().bytes().to_base64(STANDARD)
}

