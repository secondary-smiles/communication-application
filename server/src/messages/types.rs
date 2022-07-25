use serde::Deserialize;
use toml::value::*;

#[derive(Deserialize, Debug)]
pub struct Message {
    pub kind: Kind,
    pub head: Head,
    pub body: Body,
}

#[derive(Deserialize, Debug)]
pub struct Kind {
    pub kind: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Head {
    pub sent: Option<Datetime>,
    pub s_ID: String,
    pub r_ID: String,
    pub size: i64,
    pub edited: Option<bool>,
    pub kind: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Body {
    pub data: Option<String>,
    pub alert: Option<String>,
    pub w_TIME: Option<i64>,
    pub m_ID: Option<String>,
    pub m_TO: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_toml() {
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
        assert_eq!("Hey @user2!", parsed_toml.body.data.as_ref().unwrap());
    }
}
