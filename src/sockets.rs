// See the Sockets section in https://www.manpagez.com/man/5/launchd.plist/
//

use crate::error::Error;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::Deref;
use std::path::Path;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum Sockets {
    Dictionary(HashMap<String, SocketOptions>),
    Array(Vec<HashMap<String, SocketOptions>>),
}

impl From<Socket> for Sockets {
    fn from(socket: Socket) -> Self {
        Sockets::Dictionary(socket.values)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Socket {
    values: HashMap<String, SocketOptions>,
}

impl Deref for Socket {
    type Target = HashMap<String, SocketOptions>;

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl Socket {
    pub fn new<S: AsRef<str>>(name: S, options: SocketOptions) -> Self {
        Self {
            values: HashMap::from([(name.as_ref().to_string(), options)]),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq, Hash)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "PascalCase")]
pub struct SocketOptions {
    sock_type: Option<SocketType>,
    sock_passive: Option<bool>,
    sock_node_name: Option<String>,
    sock_service_name: Option<String>,
    sock_family: Option<SocketFamily>,
    sock_protocol: Option<SocketProtocol>,
    sock_path_name: Option<String>,
    secure_socket_with_key: Option<String>,
    sock_path_mode: Option<i128>,
    bonjour: Option<BonjourType>,
    multicast_group: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum SocketType {
    Dgram,
    Stream,
    Seqpacket,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "UPPERCASE")]
pub enum SocketProtocol {
    Tcp,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SocketFamily {
    IPv4,
    IPv6,
    Unix,
}

impl SocketOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_type(mut self, value: SocketType) -> Self {
        self.sock_type = Some(value);
        self
    }

    pub fn with_passive(mut self, value: bool) -> Self {
        self.sock_passive = Some(value);
        self
    }

    pub fn passive(self) -> Self {
        self.with_passive(true)
    }

    pub fn with_node_name<S: AsRef<str>>(mut self, value: S) -> Self {
        self.sock_node_name = Some(value.as_ref().to_string());
        self
    }

    pub fn with_service_name<S: AsRef<str>>(mut self, value: S) -> Self {
        self.sock_service_name = Some(value.as_ref().to_string());
        self
    }

    pub fn with_family(mut self, value: SocketFamily) -> Self {
        self.sock_family = Some(value);
        self
    }

    pub fn with_protocol(mut self, value: SocketProtocol) -> Self {
        self.sock_protocol = Some(value);
        self
    }

    pub fn with_path_name<P: AsRef<Path>>(mut self, name: P) -> Result<Self, Error> {
        let pathstr = name
            .as_ref()
            .to_str()
            .ok_or(Error::PathConversion)?
            .to_owned();
        self.sock_path_name = Some(pathstr);
        Ok(self)
    }

    pub fn with_secure_socket_key<S: AsRef<str>>(mut self, value: S) -> Self {
        self.secure_socket_with_key = Some(value.as_ref().to_string());
        self
    }

    pub fn with_path_mode(mut self, value: i128) -> Self {
        self.sock_path_mode = Some(value);
        self
    }

    pub fn with_bonjour(mut self, value: BonjourType) -> Self {
        self.bonjour = Some(value);
        self
    }

    pub fn with_multicast_group<S: AsRef<str>>(mut self, value: S) -> Self {
        self.multicast_group = Some(value.as_ref().to_string());
        self
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum BonjourType {
    Boolean(bool),
    String(String),
    Array(Vec<String>),
}

impl From<bool> for BonjourType {
    fn from(value: bool) -> Self {
        BonjourType::Boolean(value)
    }
}

impl From<String> for BonjourType {
    fn from(value: String) -> Self {
        BonjourType::String(value)
    }
}

impl From<Vec<String>> for BonjourType {
    fn from(value: Vec<String>) -> Self {
        BonjourType::Array(value)
    }
}
