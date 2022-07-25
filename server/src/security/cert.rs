#![allow(dead_code)]
use blake3;
use toml;

use serde::{Deserialize, Serialize};
use toml::value::Datetime;

use chrono::{DateTime, Duration, Utc};

use crate::security::pem;

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct Cert {
    pub key: String,
    pub signature: String,
    pub expire: Datetime,
}

impl Cert {
    pub fn toml(&self) -> String {
        toml::to_string_pretty(self).unwrap()
    }
}

pub fn new(pem: &pem::Pem) -> Cert {
    let now: DateTime<Utc> = Utc::now();
    let now_year = now
        .checked_add_signed(Duration::days(365))
        .unwrap()
        .to_rfc3339();

    let cert_preparse = format!(
        "key = '''{}'''\nsignature = ''\nexpire = {}\n",
        pem.public, now_year
    );
    let mut cert: Cert = toml::from_str(&cert_preparse).unwrap();
    let hash = blake3::hash(cert.toml().as_bytes());
    cert.signature = hash.to_string();

    cert
}

#[cfg(test)]
mod tests {
    use crate::security::{cert, pem};

    #[test]
    fn test_new_cert() {
        let pem1 = pem::new();
        let pem2 = pem::new();
        let cert1 = cert::new(&pem1);
        let cert2 = cert::new(&pem2);

        assert_ne!(cert1, cert2);
        assert_ne!(cert1.key, cert2.key);
        assert_ne!(cert1.signature, cert2.signature);
    }

    #[test]
    fn test_cert_toml() {
        let pem = pem::new();
        let cert1 = cert::new(&pem);
        let toml = cert1.toml();
        let cert2: cert::Cert = toml::from_str(&toml).unwrap();

        assert_eq!(cert1, cert2);
    }

    #[test]
    fn test_hash_verify() {
        let pem = pem::new();
        let cert = cert::new(&pem);
        let mut hash_cert = cert.clone();
        hash_cert.signature = "".to_string();
        let hash = blake3::hash(hash_cert.toml().as_bytes()).to_string();

        assert_eq!(cert.signature, hash);
        assert_ne!(
            cert.signature,
            blake3::hash(cert.toml().as_bytes()).to_string()
        );
    }
}
