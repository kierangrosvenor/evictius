use netstat2::{get_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo, TcpState};
use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, System, UpdateKind};
use windows_sys::Win32::Foundation::CloseHandle;
use windows_sys::Win32::System::Threading::{OpenProcess, PROCESS_TERMINATE};

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

// Asks Windows if we're allowed to kill this process, without killing it.
// Services and other users' processes say no unless the app runs as admin.
fn can_terminate(pid: u32) -> bool {
    unsafe {
        let handle = OpenProcess(PROCESS_TERMINATE, 0, pid);
        if handle.is_null() {
            return false;
        }
        CloseHandle(handle);
        true
    }
}

pub fn kill_process(pid: u32) -> Result<String, String> {
    let pid = Pid::from_u32(pid);

    // Only look up the one process we want, not every process on the machine.
    let mut sys = System::new();
    sys.refresh_processes(ProcessesToUpdate::Some(&[pid]), true);

    let process = sys
        .process(pid)
        .ok_or_else(|| format!("No process found with PID: {}", pid))?;

    if !process.kill() {
        return Err(format!("Failed to kill process with PID: {} (it may need admin rights)", pid));
    }

    Ok(format!("Successfully killed process with PID: {}", pid))
}

pub fn fetch_processes() -> Result<Vec<PortInfo>, String> {
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
        .filter(|&(_, pid)| can_terminate(pid))
        .collect();

    // Only load what we show: names come for free, the command has to be asked for.
    let mut sys = System::new();
    sys.refresh_processes_specifics(
        ProcessesToUpdate::All,
        true,
        ProcessRefreshKind::nothing().with_cmd(UpdateKind::OnlyIfNotSet),
    );

    let mut ports: Vec<PortInfo> = listening
        .into_iter()
        .filter_map(|(port, pid)| {
            let info = sys.process(Pid::from_u32(pid))?;
            let process = info.name().to_string_lossy().into_owned();
            let parts: Vec<String> = info
                .cmd()
                .iter()
                .map(|part| part.to_string_lossy().into_owned())
                .collect();
            let command = parts.join(" ");
            let short_command = super::shorten_command(&parts);
            Some(PortInfo { port, pid, process, command, short_command })
        })
        .filter(|p| !SYSTEM_PROCESSES.contains(&p.process.to_lowercase().as_str()))
        .collect();

    ports.sort_by_key(|p| p.port);
    ports.dedup_by_key(|p| p.port);
    Ok(ports)
}
