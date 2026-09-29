#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PortInfo {
    pub port: u16,
    pub pid: u32,
    pub process: String,
    pub command: String,
    pub short_command: String,
}

pub fn shorten_command(parts: &[String]) -> String {
    if let Some(package) = parts.iter().find_map(|p| node_package(p)) {
        return package;
    }
    parts.iter().map(|p| file_name(p)).collect::<Vec<_>>().join(" ")
}

fn segments(path: &str) -> Vec<&str> {
    path.split(['\\', '/']).filter(|s| !s.is_empty() && *s != ".").collect()
}

fn node_package(path: &str) -> Option<String> {
    let segments = segments(path);
    let start = segments.iter().rposition(|s| *s == "node_modules")? + 1;
    let rest: Vec<&str> = segments[start..]
        .iter()
        .copied()
        .filter(|s| *s != ".bin" && *s != "..")
        .collect();
    match rest.as_slice() {
        [scope, name, ..] if scope.starts_with('@') => Some(format!("{scope}/{name}")),
        [name, ..] => Some(name.to_string()),
        [] => None,
    }
}

fn file_name(part: &str) -> String {
    if part.starts_with('-') || part.contains("://") {
        return part.to_string();
    }
    let name = segments(part).last().copied().unwrap_or(part);
    match name.len().checked_sub(4) {
        Some(i) if name[i..].eq_ignore_ascii_case(".exe") => name[..i].to_string(),
        _ => name.to_string(),
    }
}

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::{fetch_processes, kill_process};

#[cfg(not(target_os = "windows"))]
pub fn fetch_processes() -> Result<Vec<PortInfo>, String> {
    Err("fetch_processes is not supported on this platform yet".into())
}

#[cfg(not(target_os = "windows"))]
pub fn kill_process(pid: u32) -> Result<String, String> {
    Err(format!("kill_process is not supported on this platform yet (PID: {})", pid))
}

#[cfg(test)]
mod tests {
    use super::shorten_command;

    fn short(parts: &[&str]) -> String {
        shorten_command(&parts.iter().map(|s| s.to_string()).collect::<Vec<_>>())
    }

    #[test]
    fn shortens_commands() {
        assert_eq!(short(&["node", r"C:\proj\node_modules\.bin\..\vite\bin\vite.js"]), "vite");
        assert_eq!(short(&["node", "/proj/node_modules/@astrojs/cli/bin.js", "dev"]), "@astrojs/cli");
        assert_eq!(short(&[r"C:\Python\python.exe", "-m", "http.server"]), "python -m http.server");
        assert_eq!(short(&[r"C:\Program Files\app\App.EXE", "--url=http://x/y"]), "App --url=http://x/y");
    }
}
