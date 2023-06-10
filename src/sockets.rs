// See the Sockets section in https://www.manpagez.com/man/5/launchd.plist/
//

use crate::error::Error;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::Deref;
use std::path::Path;

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[derive(Debug, Clone, PartialEq)]
pub enum Sockets {
    Dictionary(HashMap<String, SocketOptions>),
    Array(Vec<HashMap<String, SocketOptions>>),
}

impl From<Socket> for Sockets {
    fn from(socket: Socket) -> Self {
        Sockets::Dictionary(socket.values)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq)]
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

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "io", serde(rename_all = "PascalCase"))]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
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
    bonjour: Option<BonjourTypes>,
    multicast_group: Option<String>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SocketType {
    Dgram,
    Stream,
    Seqpacket,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "UPPERCASE"))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SocketProtocol {
    Tcp,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SocketFamily {
    IPv4,
    IPv6,
    Unix,
}

impl SocketOptions {
    pub fn new() -> Self {
        Self {
            sock_type: None,
            sock_passive: None,
            sock_node_name: None,
            sock_service_name: None,
            sock_family: None,
            sock_protocol: None,
            sock_path_name: None,
            secure_socket_with_key: None,
            sock_path_mode: None,
            bonjour: None,
            multicast_group: None,
        }
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

    pub fn with_bonjour(mut self, value: BonjourTypes) -> Self {
        self.bonjour = Some(value);
        self
    }

    pub fn with_multicast_group<S: AsRef<str>>(mut self, value: S) -> Self {
        self.multicast_group = Some(value.as_ref().to_string());
        self
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BonjourTypes {
    Boolean(bool),
    String(String),
    Array(Vec<String>),
}

impl From<bool> for BonjourTypes {
    fn from(value: bool) -> Self {
        BonjourTypes::Boolean(value)
    }
}

impl From<String> for BonjourTypes {
    fn from(value: String) -> Self {
        BonjourTypes::String(value)
    }
}

impl From<Vec<String>> for BonjourTypes {
    fn from(value: Vec<String>) -> Self {
        BonjourTypes::Array(value)
    }
}
