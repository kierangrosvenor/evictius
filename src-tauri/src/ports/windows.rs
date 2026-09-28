use netstat2::{get_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo, TcpState};
use sysinfo::{Pid, ProcessesToUpdate, System};

use super::PortInfo;


const DYNAMIC_PORT_START: u16 = 49152; // Windows hands out ports from this number up to its own services.
const SYSTEM_PROCESSES: &[&str] = &[ // Ports and services we don't care about, because they are part of the Windows
    "system",
    "svchost.exe",
    "lsass.exe",
    "wininit.exe",
    "services.exe",
    "spoolsv.exe",
];

pub fn fetch_ports() -> Result<Vec<PortInfo>, String> {
    let af = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let sockets = get_sockets_info(af, ProtocolFlags::TCP).map_err(|e| e.to_string())?;

    let listening: Vec<(u16, u32)> = sockets
        .into_iter()
        .filter(|s| {
            matches!(
                &s.protocol_socket_info,
                ProtocolSocketInfo::Tcp(tcp) if tcp.state == TcpState::Listen
            )
        })
        .filter(|s| s.local_port() < DYNAMIC_PORT_START)
        .filter_map(|s| Some((s.local_port(), *s.associated_pids.first()?)))
        .collect();

    let mut sys = System::new();
    sys.refresh_processes(ProcessesToUpdate::All, true);

    let mut ports: Vec<PortInfo> = listening
        .into_iter()
        .filter_map(|(port, pid)| {
            let process = sys
                .process(Pid::from_u32(pid))?
                .name()
                .to_string_lossy()
                .into_owned();
            Some(PortInfo { port, pid, process })
        })
        .filter(|p| !SYSTEM_PROCESSES.contains(&p.process.to_lowercase().as_str()))
        .collect();

    ports.sort_by_key(|p| p.port);
    ports.dedup_by_key(|p| p.port);
    Ok(ports)
}
