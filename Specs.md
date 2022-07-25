# Communication-Application Specifications

> These are the specifications for a communication application to follow, using TCP Sockets and [TOML](https://toml.io).

# Types of Messages

## Client Received Format

### Visible-user

Visible-user messages are standard messages sent from one user to another. 

These messages that will be directly shown to the user

### Visible-server

Visible-server messages are messages from the server to a user.

These are things like cooldowns, or system updates.

### Context

These are messages that add context to messages if required.

Context messages are things like edits to an existing message, or a notice to the client to delete a message.

**Note: Context messages can also be used for things like terminating the socket connection.**

### Visible-user Messages

#### Headers

> \* required

- Time/Date sent -- (UTC)

- Sender ID -- (hash) *

- Receiver ID -- (hash) *

- Size -- (bytes) *

- Edited -- (yes or no) (* if yes)

- Data Type -- (text, image, file, link, etc.)

#### Body

> \* required

- [...data] *
- Message ID -- (hash) *

### Visible-server Messages

#### Headers

> \* required

- Time/Date sent -- (UTC)

- Server ID -- (rather than Sender ID) *

- Receiver ID -- (hash) *

- Size -- (bytes) *

- Data Type -- (text, image, file, link, etc.)

#### Body

> \* required

- [...data] *

### Context Messages

#### Headers

> \* required

- Time/Date sent -- (UTC)

- Server ID -- (rather than Sender ID) *

- Receiver ID -- (hash) *

- Size -- (bytes) *

- Data Type -- (text, image, file, link, etc.)

- Context Type -- (message update, deletion notice, read receipt, etc.) *

#### Body

> \* required

- [...data] *

# Format

> These are specifications for the global format of a message

Data is sent via TOML file

```toml
[type]
# Message type
#...

[head]
# Header data
#...

[body]
# Body data
#...
```

## Visible-user Message Examples

```toml
[kind]
kind = "VIS-USER"

[head]
sent = 1979-05-27T07:32:00Z
s_ID = "USER-b3daa77b4c04a9551b8781d03191fe098f325e67" # Sender ID is a SHA1 hash
r_ID = "USER-a1881c06eec96db9901c7bbfe41c42a3f08e9cb4" # Receiver ID is also a SHA1 hash
size = 0x14 # Hexadecimal for conciseness
edited = false
kind = "text/plain" # A custom subset of MIME Types are used for the message type

[body]
data = 'Hey @user2!'
```

```toml
[kind]
kind = "VIS-USER"

[head]
sent = 1979-05-27T07:32:00Z
s_ID = "USER-b3daa77b4c04a9551b8781d03191fe098f325e67"
r_ID = "USER-a1881c06eec96db9901c7bbfe41c42a3f08e9cb4"
size = 0x25
edited = false
kind = "image/png"

[body]
data = "https://[server].com/[image]" # Images are uploaded to the server and the client fetches it to display
```

## Visible-server Message Examples

```toml
[kind]
kind = "VIS-USER"

[head]
sent = 1979-05-27T07:32:00Z
s_ID = "SERVER-3de4f901fffb30ac720b0e7eb654b4faa2dd03fa" # This identifies the server as a real server rather than just another user
r_ID = "USER-b3daa77b4c04a9551b8781d03191fe098f325e67"
size = 0x28
kind = "alert/join" # This is where we deviate from MIME Types

[body]
alert = "@user2 Just joined the thread!" # This data will be shown in a special type of message by the client
```

```toml
[kind]
kind = "VIS-USER"

[head]
sent = 1979-05-27T07:32:00Z
s_ID = "SERVER-3de4f901fffb30ac720b0e7eb654b4faa2dd03fa" # This identifies the server as a real server rather than just another user
r_ID = "USER-b3daa77b4c04a9551b8781d03191fe098f325e67"
size = 0x44
kind = "alert/cooldown"

[body]
alert = "You've been put on cooldown for 30 seconds!"
w_TIME = 30000 # w_TIME is the time in milliseconds that the user will be forced to wait
```

## Context Message Examples

```toml
[kind]
kind = "CTXT"

[head]
sent = 1979-05-27T07:32:00Z
s_ID = "SERVER-3de4f901fffb30ac720b0e7eb654b4faa2dd03fa"
r_ID = "USER-b3daa77b4c04a9551b8781d03191fe098f325e67"
size = 0x57
kind = "update/delete"

[body]
alert= "@user1 deleted a message" # This alert is shown to the user in a style similar to Visible-server messages
m_ID = "MSG-2ae4a9acbe4e1fa22eab24e32674167a65ff14fd" # In this case, m_ID is a reference to the message id to find and remove
```

```toml
[kind]
kind = "CTXT"

[head]
sent = 1979-05-27T07:32:00Z
s_ID = "SERVER-3de4f901fffb30ac720b0e7eb654b4faa2dd03fa"
r_ID = "USER-b3daa77b4c04a9551b8781d03191fe098f325e67"
size = 0x60
kind = "update/edit"

[body]
m_ID = "MSG-2ae4a9acbe4e1fa22eab24e32674167a65ff14fd"
m_TO = "Hey! (sorry I typed that wrong)" # In this case, m_TO is what the client should correct the message at m_ID to
```

# Data Flow

> Messages follow a specific sequence of steps before being viewed by the person they were sent to.

### Visible-user

1) User submits a message.

2) Sending client constructs the TOML message with the `type`, `r_ID`, `size`, `type` and `body` fields.

3) TOML message is sent to server.

4) Server adds the `sent` and `s_ID` fields.

5) Server sends the updated TOML message to the receiving client.

6) Receiving client parses the TOML message and displays it to the client.

7) Receiving client sends an `update/delivered` Context to server.

8) Server forwards an `update/delivered` context to sending client.

9) Sending client displays a received context to the user.

### Visible-server

1) Server constructs TOML message.

2) TOML message is sent to the receiving client.

3) Receiving client parses the TOML message.

4) Receiving client displays the context to the user.

### Context

1) Server constructs TOML message.

2) TOML message is sent to the receiving client.

3) Receiving client parses the TOML message.

4) Receiving client displays the context to the user.

# Security

> How will messages be sent from the server to the client and vice-versa in a way that is secure and keeps the users data private?

Security will be based off the information in the image. [An SSL Conversation Between Computer and Server](https://imgur.com/gallery/5T2fJsG)

## The Secure Loop Handshake

> This details the secure loop that will be used with every connection to determine both parties are who they claim to be.

1. The client connects with the server and requests its public identity.

2. The server sends its public identity over.

3. The client reads the ID and compares the hash and key with its own copy.

4. If the client can verify the server, it will encrypt its own ID and a symmetrical key that will be used for further messages between the server and client.

5. The server will attempt to identify the client and if it can, it will continue the connection to the normal interface.
