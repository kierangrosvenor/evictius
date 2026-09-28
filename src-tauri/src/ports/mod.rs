#[cfg(target_os = "windows")]
pub mod windows;

#[derive(serde::Serialize)]
pub struct PortInfo {
    pub port: u16,
    pub pid: u32,
    pub process: String,
}

pub fn fetch_ports() -> Result<Vec<PortInfo>, String> {
    windows::fetch_ports()
}
