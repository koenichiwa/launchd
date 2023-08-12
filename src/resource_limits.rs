use serde::{Deserialize, Serialize};

/// Configuration options for resource limits.
/// 
/// See the HardResourceLimits / SoftResourceLimits section of <https://www.manpagez.com/man/5/launchd.plist/>
#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "PascalCase")]
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
    /// Creates a new `ResourceLimits` instance with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the resource limit for core file size in bytes.
    pub fn with_core(mut self, value: u64) -> Self {
        self.core = Some(value);
        self
    }

    /// Sets the resource limit for CPU usage time in seconds.
    pub fn with_cpu(mut self, value: u64) -> Self {
        self.cpu = Some(value);
        self
    }

    /// Sets the resource limit for data segment size.
    pub fn with_data(mut self, value: u64) -> Self {
        self.data = Some(value);
        self
    }

    /// Sets the resource limit for maximum file size in bytes.
    pub fn with_file_size(mut self, value: u64) -> Self {
        self.file_size = Some(value);
        self
    }

    /// Sets the resource limit for locked memory size in bytes.
    pub fn with_memory_lock(mut self, value: u64) -> Self {
        self.memory_lock = Some(value);
        self
    }

    /// Sets the resource limit for maximum number of open files.
    pub fn with_number_of_files(mut self, value: u64) -> Self {
        self.number_of_files = Some(value);
        self
    }

    /// Sets the resource limit for maximum number of processes.
    pub fn with_number_of_processes(mut self, value: u64) -> Self {
        self.number_of_processes = Some(value);
        self
    }

    /// Sets the resource limit for resident set size in bytes.
    pub fn with_resident_set_size(mut self, value: u64) -> Self {
        self.resident_set_size = Some(value);
        self
    }

    /// Sets the resource limit for stack size in bytes.
    pub fn with_stack(mut self, value: u64) -> Self {
        self.stack = Some(value);
        self
    }
}
