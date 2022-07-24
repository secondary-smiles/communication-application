#![allow(dead_code)]
use openssl::rsa::{Padding, Rsa};

#[derive(Debug, PartialEq)]
pub struct Pem {
    pub public: String,
    private: String,
}

impl Pem {
    pub fn encrypt(&self, data: &str) -> Vec<u8> {
        let rsa = Rsa::public_key_from_pem(self.public.as_bytes()).unwrap();
        let mut buf: Vec<u8> = vec![0; rsa.size() as usize];
        let _ = rsa
            .public_encrypt(data.as_bytes(), &mut buf, Padding::PKCS1)
            .unwrap();
        buf
    }

    pub fn decrypt(&self, data: Vec<u8>) -> String {
        let rsa = Rsa::private_key_from_pem(self.private.as_bytes()).unwrap();
        let mut buf: Vec<u8> = vec![0; rsa.size() as usize];
        let _ = rsa
            .private_decrypt(&data, &mut buf, Padding::PKCS1)
            .unwrap();
        let res = String::from_utf8(buf)
            .unwrap()
            .trim_end_matches(char::from(0))
            .to_string();
        res
    }

    pub fn private(&self) -> String {
        let p = &self.private;
        p.to_string()
    }

    pub fn data(&self) -> String {
        let r = format!("{}\n{}", self.public, self.private);
        r
    }
}

pub fn new() -> Pem {
    let rsa = Rsa::generate(1024).unwrap();
    let private_key: Vec<u8> = rsa.private_key_to_pem().unwrap();
    let public_key: Vec<u8> = rsa.public_key_to_pem().unwrap();

    Pem {
        public: String::from_utf8(public_key).unwrap(),
        private: String::from_utf8(private_key).unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use crate::security::pem;

    #[test]
    fn test_new_pem() {
        let pem1 = pem::new();
        let pem2 = pem::new();

        assert_ne!(pem1, pem2)
    }

    #[test]
    fn test_pem_encrypt_and_decrypt() {
        let pem = pem::new();
        let data = "The quick brown fox jumps over the lazy dog";
        let e = pem.encrypt(data);
        let e_string = format!("{:?}", e);
        let d = pem.decrypt(e);

        assert_ne!(e_string, format!("{:?}", data));
        assert_eq!(d, data);
    }
}
