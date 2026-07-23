use serde::{Deserialize, Serialize};
use sysinfo::System;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OperatingSystemInfo {
    pub name: String,
    pub version: String,
    pub kernel_version: String,
    pub host_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CpuInfo {
    pub brand: String,
    pub architecture: String,
    pub logical_cores: usize,
    pub physical_cores: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MemoryInfo {
    pub total_bytes: u64,
    pub available_bytes: u64,
    pub used_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SystemSnapshot {
    pub operating_system: OperatingSystemInfo,
    pub cpu: CpuInfo,
    pub memory: MemoryInfo,
}

pub struct SystemProbe;

impl SystemProbe {
    pub fn collect() -> SystemSnapshot {
        let mut system = System::new_all();
        system.refresh_all();

        let cpu_brand = system
            .cpus()
            .first()
            .map(|cpu| cpu.brand().trim().to_owned())
            .filter(|brand| !brand.is_empty())
            .unwrap_or_else(|| "Unknown CPU".to_owned());

        SystemSnapshot {
            operating_system: OperatingSystemInfo {
                name: System::name().unwrap_or_else(|| "Unknown".to_owned()),
                version: System::os_version().unwrap_or_else(|| "Unknown".to_owned()),
                kernel_version: System::kernel_version()
                    .unwrap_or_else(|| "Unknown".to_owned()),
                host_name: System::host_name().unwrap_or_else(|| "Unknown".to_owned()),
            },
            cpu: CpuInfo {
                brand: cpu_brand,
                architecture: std::env::consts::ARCH.to_owned(),
                logical_cores: system.cpus().len(),
                physical_cores: System::physical_core_count(),
            },
            memory: MemoryInfo {
                total_bytes: system.total_memory(),
                available_bytes: system.available_memory(),
                used_bytes: system.used_memory(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SystemProbe;

    #[test]
    fn collects_a_valid_system_snapshot() {
        let snapshot = SystemProbe::collect();

        assert!(!snapshot.operating_system.name.is_empty());
        assert!(!snapshot.cpu.architecture.is_empty());
        assert!(snapshot.cpu.logical_cores > 0);
        assert!(snapshot.memory.total_bytes >= snapshot.memory.used_bytes);
    }
}
