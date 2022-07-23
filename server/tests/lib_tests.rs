use server::*;

#[test]
fn test_message_structs() {
    let toml = r#"
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
"#;
    let parsed_toml: Message = toml::from_str(toml).unwrap();

    assert_eq!("VIS-USER", parsed_toml.kind.kind);
    assert_eq!("text/plain", parsed_toml.head.kind);
    assert_eq!(20, parsed_toml.head.size);
    assert_eq!("Hey @user2!", parsed_toml.body.data.as_ref().unwrap());
}
