#![allow(dead_code)]
use blake3;
use toml;

use serde::Deserialize;
use toml::value::Datetime;

use chrono::{DateTime, Duration, Utc};

use crate::security::pem;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Cert {
    pub key: String,
    pub hash: String,
    pub expire: Datetime,
}

pub fn new(pem: &pem::Pem) -> Cert {
    let now: DateTime<Utc> = Utc::now();
    let now_year = now
        .checked_add_signed(Duration::days(365))
        .unwrap()
        .to_rfc3339();

    let cert_preparse = format!(
        "key = '''{}'''\nhash = ''\nexpire = {}",
        pem.public, now_year
    );
    let mut cert: Cert = toml::from_str(&cert_preparse).unwrap();
    let prehash = format!("{:?}", cert);
    let prehash_hash = blake3::hash(prehash.as_bytes());

    cert.hash = prehash_hash.to_string();
    cert
}

pub fn secure_new(pem: &pem::Pem) -> Cert {
    let now: DateTime<Utc> = Utc::now();
    let now_year = now
        .checked_add_signed(Duration::days(365))
        .unwrap()
        .to_rfc3339();

    let cert_preparse = format!(
        "key = '''{}'''\nhash = ''\nexpire = {}",
        pem.public, now_year
    );
    let mut cert: Cert = toml::from_str(&cert_preparse).unwrap();
    let prehash = format!("{:?}", cert);
    let prehash_hash = blake3::hash(prehash.as_bytes());
    cert.hash = prehash_hash.to_string();
    let posthash = format!("{:?}", cert);
    let posthash_hash = blake3::hash(posthash.as_bytes());

    cert.hash = posthash_hash.to_string();

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
        assert_ne!(cert1.hash, cert2.hash);
    }

    #[test]
    fn test_secure_new_cert() {
        let pem1 = pem::new();
        let pem2 = pem::new();
        let cert1 = cert::secure_new(&pem1);
        let cert2 = cert::secure_new(&pem2);

        assert_ne!(cert1, cert2);
        assert_ne!(cert1.key, cert2.key);
        assert_ne!(cert1.hash, cert2.hash);
    }
}
