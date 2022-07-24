# Server

> These are specifications for a server loop to follow in accordance with [Specs.md](./Specs.md)

# Security

The server will use a similar method to SSL certificates as outlined in [Specs.md](./Specs.md).

1) The client will request the server's public key.

2) The server will send over the entire encrypted signature and public key as one file.

3) The client will decrypt the signature using the given public key and then compare the hashes.

4) The hashes will be the signature sent by the server and the public key as sent by the server.

5) If the client finds the server to be trustworthy, they will generate a one-time symmetric key and encrypt that with the server's public key and send it to the server. That key will be used to encrypt all further traffic between the server and client from then on.

# Server Loop

>  These are specifications for a server loop to follow to handle all clients simultaneously.

*Still figuring out the best system*
