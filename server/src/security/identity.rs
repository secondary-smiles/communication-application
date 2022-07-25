#![allow(dead_code)]
use crate::security::{cert, pem};

use serde::{Deserialize, Serialize};
use toml;

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Identity {
    pub public: Public,
    private: Private,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Public {
    pub id: String,
    pub cert: cert::Cert,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
struct Private {
    pem: pem::Pem,
}

impl Identity {
    pub fn toml(&self) -> String {
        toml::to_string_pretty(self).unwrap()
    }
}

pub fn new() -> Identity {
    let pem = pem::new();
    let cert = cert::new(&pem);
    let id = "".to_string();

    let mut identity = Identity {
        public: Public { cert, id },
        private: Private { pem },
    };

    let hash = blake3::hash(identity.toml().as_bytes());
    identity.public.id = hash.to_string();

    identity
}

#[cfg(test)]
mod tests {
    use crate::security::identity;
    use blake3;
    use toml;

    #[test]
    fn test_create_id() {
        let id1 = identity::new();
        let id2 = identity::new();

        assert_ne!(id1.public.cert.signature, id2.public.cert.signature);
        assert_ne!(id1.private.pem.data(), id2.private.pem.data());
    }

    #[test]
    fn test_id_toml() {
        let id1 = identity::new();
        let toml = id1.toml();
        let id2: identity::Identity = toml::from_str(&toml).unwrap();

        assert_eq!(id1, id2);
    }

    #[test]
    fn test_hash_verify() {
        let id = identity::new();
        let mut hash_id = id.clone();
        hash_id.public.id = "".to_string();
        let hash = blake3::hash(hash_id.toml().as_bytes()).to_string();

        assert_eq!(id.public.id, hash);
        assert_ne!(id.public.id, blake3::hash(id.toml().as_bytes()).to_string());
    }
}
