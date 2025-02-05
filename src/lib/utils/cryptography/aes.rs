// block-modes = "0.8.1"
// block-padding = "0.2.1"
// aes = "0.7.5"
// base64 = "0.22.0"
use aes::Aes256;
use base64::prelude::*;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};
use worker::*;

pub type Aes256Cbc = Cbc<Aes256, Pkcs7>;

pub trait CbcNonce {
    fn new(key: &str, iv: &str) -> Result<Self>
    where
        Self: Sized;
    fn decipher(self, input: &[u8]) -> Result<Vec<u8>>;
    fn cipher(self, input: &[u8]) -> String;
    fn cipher_str(self, input: &str) -> String;
}

impl CbcNonce for Aes256Cbc {
    fn new(key: &str, iv: &str) -> Result<Self> {
        let key_vec = key[..32].as_bytes().to_vec();
        let iv_vec = iv[iv.len() - 16..].as_bytes().to_vec();

        match Aes256Cbc::new_from_slices(&key_vec, &iv_vec) {
            Ok(payload) => Ok(payload),
            Err(x) => Err(worker::Error::RustError(format!("test{}", x))),
        }
    }

    fn decipher(self, input: &[u8]) -> Result<Vec<u8>> {
        let encrypted = match BASE64_STANDARD.decode(input) {
            Ok(x) => Ok(x),
            Err(err) => Err(Error::RustError(err.to_string())),
        }?;

        match self.decrypt_vec(&encrypted) {
            Ok(payload) => Ok(payload),
            Err(x) => Err(Error::RustError(x.to_string())),
        }
    }

    fn cipher(self, input: &[u8]) -> String {
        BASE64_STANDARD.encode(self.encrypt_vec(input))
    }

    fn cipher_str(self, input: &str) -> String {
        self.cipher(input.as_bytes())
    }
}
