// See the HardResourceLimits / SoftResourceLimits section of https://www.manpagez.com/man/5/launchd.plist/
//
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "io", serde(rename_all = "PascalCase"))]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ResourceLimits {
    core: Option<u64>,
    #[serde(rename = "CPU")]
    cpu: Option<u64>,
    data: Option<u64>,
    file_size: Option<u64>,
    memory_lock: Option<u64>,
    number_of_files: Option<u64>,
    number_of_processes: Option<u64>,
    resident_set_size: Option<u64>,
    stack: Option<u64>,
}

impl ResourceLimits {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn with_core(mut self, value: u64) -> Self {
        self.core = Some(value);
        self
    }

    pub fn with_cpu(mut self, value: u64) -> Self {
        self.cpu = Some(value);
        self
    }

    pub fn with_data(mut self, value: u64) -> Self {
        self.data = Some(value);
        self
    }

    pub fn with_file_size(mut self, value: u64) -> Self {
        self.file_size = Some(value);
        self
    }

    pub fn with_memory_lock(mut self, value: u64) -> Self {
        self.memory_lock = Some(value);
        self
    }

    pub fn with_number_of_files(mut self, value: u64) -> Self {
        self.number_of_files = Some(value);
        self
    }

    pub fn with_number_of_processes(mut self, value: u64) -> Self {
        self.number_of_processes = Some(value);
        self
    }

    pub fn with_resident_set_size(mut self, value: u64) -> Self {
        self.resident_set_size = Some(value);
        self
    }

    pub fn with_stack(mut self, value: u64) -> Self {
        self.stack = Some(value);
        self
    }
}
