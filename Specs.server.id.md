# Identity Specifications

> These are specifications for a id format that will be sent between server and client to verify server identity.

# Overview

Security will be enforced using a system similar to SSL certificates.

Every server and client will have an Identity.

# Identity

> Specs for the internals of every server and client ID

Each ID is made up of two parts, the Pem key and the Cert.

## Pem

The Pem key is a TOML file containing the public and private keys that will be used for encryption and decryption. 

The actual implementation uses OpenSSL to generate the key pair.

An example of a Pem file as TOML

```toml
public = '''
-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQCvJmlKdzEZHZxlKNAQc8FiHHkF
dcDTOdEvwUdIrLLkxvzHggnqYUyt22VF3PVCtaKDzNwMnwgUylp4GdpywzCTeX7P
Gx8ytOLTJfTrDUY/i/6ywPsCI+qQ6C0/bBupcgYjYWax25dNN5vCvcdZ98LAf0G4
OIPfwIx1ZnuRqzuxswIDAQAB
-----END PUBLIC KEY-----
'''
private = '''
-----BEGIN RSA PRIVATE KEY-----
MIICWwIBAAKBgQCvJmlKdzEZHZxlKNAQc8FiHHkFdcDTOdEvwUdIrLLkxvzHggnq
YUyt22VF3PVCtaKDzNwMnwgUylp4GdpywzCTeX7PGx8ytOLTJfTrDUY/i/6ywPsC
I+qQ6C0/bBupcgYjYWax25dNN5vCvcdZ98LAf0G4OIPfwIx1ZnuRqzuxswIDAQAB
AoGAe/aDruCNL2l1t2MbX4mN5RKh1Otyfgz9sP7qpNUWCDWN0uaGfgw/aQ1OsBOi
woQrcX0cgRJNdemB/l7N7UuYpJsSx5V+SSI2FUuqW6E950CCwI+xL2iyQ3BSV+VA
cZplnuHU+IY2bUQJi3tjV7t9lokmriUOIe0HJ4yr8VkhgYECQQDmqMnmarXcLXBc
6qteccZAp+MZ+P3+j2D0tOgrFuIo5couVThiEIg1pBVismbKtx6j7IGzcInsKOX6
aPm0wcmPAkEAwmRxmbHvM1Cs3Vo2ycyRUoZm/vOJ9rii//pISc2jMmRLB4MRTMLM
dLplLquVUBLcuEpmOpDHruWcNJjUg08bnQJAGfwT2rBU0nwP93e8XIRmuCvxqqBh
RMc0Pf0PVqNaUQ8qCrQxCOMh5SEaA6djaxzcB82CAwv/japB1kbzMojOQwJAYpzN
vCg1rhR4/Pls3cY22mRwD1nAXTahCbYPwdDdvwx9u1vUBNq1GWoNNjW40xweeCbV
4VPc2G6GZXA6LjqNSQJAdgBMUMgWkmSd0abzOAzf1DTquxugRtCDNJ/jC3APfoIQ
a0sEgz4WFeYcbWL3dMRJ7UCJNa4utUTQnwDWF2oygg==
-----END RSA PRIVATE KEY-----
'''
```

## Cert

Similar to Pem, Cert is essentially, the public facing aspect of an Identity.

The basic structure of a Cert is this

```toml
key = "" # The public sde of a Pem key
hash = "" # The hash of the entire Cert
expire = "" # The date this Cert expires and new one must be generated for the client
```

- `Key` is the public key of a the Pem.

- `Hash` is a hash of the entire Cert.

- `Expire` is the date on that the Cert will have to be recalculated by.

## Identity

The Identity is a combination of the Pem and Cert plus an ID. It looks like this

```toml
[public]
id = "" # This is the unique id of the Identity

[public.cert]
key = ""
hash = ""
expire = ""

[private.pem]
public = ''
private = ''
```

### ID

The ID is a hash of the server Identity minus the ID, similar to how the `hash` field works in a Cert.
