use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::Deref;
use std::path::PathBuf;

/// Enumeration of different socket configurations.
///
/// See the Sockets section in <https://www.manpagez.com/man/5/launchd.plist/>
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum Sockets {
    /// Represents socket options as a dictionary of named socket configurations.
    Dictionary(HashMap<String, SocketOptions>),
    /// Represents socket options as an array of socket configurations.
    Array(Vec<HashMap<String, SocketOptions>>),
}

impl From<Socket> for Sockets {
    fn from(socket: Socket) -> Self {
        Sockets::Dictionary(socket.values)
    }
}

/// Socket configuration container.
///
/// This struct holds a collection of named socket options.
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
    /// Creates a new `Socket` instance with a single socket configuration.
    pub fn new(name: String, options: SocketOptions) -> Self {
        Self {
            values: HashMap::from([(name, options)]),
        }
    }
}

/// Configuration options for socket behavior.
///
/// This struct holds various socket options that can be configured for a service.
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
    sock_path_name: Option<PathBuf>,
    secure_socket_with_key: Option<String>,
    sock_path_mode: Option<i128>,
    bonjour: Option<BonjourType>,
    multicast_group: Option<String>,
}

/// Enumeration of different socket types.
///
/// This enum represents different socket types that can be configured.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum SocketType {
    Dgram,
    Stream,
    Seqpacket,
}

/// Enumeration of different socket protocols.
///
/// This enum represents different socket protocols that can be configured.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "UPPERCASE")]
pub enum SocketProtocol {
    Tcp,
}

/// Enumeration of different socket families.
///
/// This enum represents different socket families that can be configured.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SocketFamily {
    IPv4,
    IPv6,
    Unix,
}

impl SocketOptions {
    /// Creates a new `SocketOptions` instance with default values.
    pub fn new() -> Self {
        Self::default()
    }

    // Functions for setting socket options...

    // ...
}

/// Enumeration of different Bonjour types.
///
/// This enum represents different types of Bonjour configurations.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum BonjourType {
    /// Represents a boolean value for Bonjour configuration.
    Boolean(bool),
    /// Represents a string value for Bonjour configuration.
    String(String),
    /// Represents an array of string values for Bonjour configuration.
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
