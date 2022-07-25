#![allow(dead_code)]
use crate::security::{cert, pem};

use serde::{Deserialize, Serialize};
use toml;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Identity {
    pub public: Public,
    private: Private,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Public {
    pub id: String,
    pub cert: cert::Cert,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct Private {
    pem: pem::Pem,
}

pub fn new() -> Identity {
    let pem = pem::new();
    let cert = cert::new(&pem);
    let id = "".to_string();

    let mut identity = Identity {
        public: Public { cert, id },
        private: Private { pem },
    };

    let toml_for_hash = toml::to_string(&identity).unwrap();
    let hash = blake3::hash(toml_for_hash.as_bytes());
    identity.public.id = hash.to_string();
    let toml_for_hash = toml::to_string(&identity).unwrap();
    let hash = blake3::hash(toml_for_hash.as_bytes());
    identity.public.id = hash.to_string();

    identity
}

#[cfg(test)]
mod tests {
    use crate::security::identity;

    #[test]
    fn test_create_id() {
        let id1 = identity::new();
        let id2 = identity::new();

        assert_ne!(id1.public.cert.hash, id2.public.cert.hash);
        assert_ne!(id1.private.pem.data(), id2.private.pem.data());
    }
}