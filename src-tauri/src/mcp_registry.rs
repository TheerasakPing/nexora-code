//! Nexora-owned MCP server inventory. Configuration only: no process spawn,
//! HTTP connection, automatic tool permission or secret import occurs here.
//! Provider-managed MCP configuration remains independent.
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, OpenOptions},
    io::{Read, Write},
    path::{Component, Path, PathBuf},
    sync::{Arc, Mutex},
};
use tauri::State;

const MAX_SERVERS: usize = 64;
const MAX_FILE_BYTES: u64 = 256 * 1024;
const MAX_ARGUMENTS: usize = 48;
const MAX_STRING: usize = 4096;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct McpServer {
    pub name: String,
    pub description: String,
    pub enabled: bool,
    pub transport: Transport,
    // Reference to a separate OS secure-store item, not a credential or
    // authorization grant. The renderer never receives the secret value.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "credentialRef")]
    pub credential_ref: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Transport {
    Stdio { command: String, args: Vec<String> },
    StreamableHttp { url: String },
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Inventory {
    version: u32,
    servers: Vec<McpServer>,
}

#[derive(Clone)]
pub struct Registry {
    file: PathBuf,
    lock: Arc<Mutex<()>>,
}

impl Registry {
    pub fn new(app_data: &Path) -> Self {
        Self {
            file: app_data.join("nexora-mcp-registry-v1.json"),
            lock: Arc::new(Mutex::new(())),
        }
    }

    fn read(&self) -> Result<Vec<McpServer>, String> {
        if !self.file.exists() {
            // A dangling symlink is not an empty registry.
            if self.file.symlink_metadata().is_ok() {
                return Err("MCP registry path is not a regular file.".into());
            }
            return Ok(Vec::new());
        }
        let meta = self
            .file
            .symlink_metadata()
            .map_err(|_| "Cannot inspect MCP registry.")?;
        if !meta.is_file() || meta.file_type().is_symlink() || meta.len() > MAX_FILE_BYTES {
            return Err("MCP registry must be a bounded regular file.".into());
        }
        let file = fs::File::open(&self.file).map_err(|_| "Cannot open MCP registry.")?;
        let mut bytes = Vec::new();
        file.take(MAX_FILE_BYTES + 1)
            .read_to_end(&mut bytes)
            .map_err(|_| "Cannot read MCP registry.")?;
        if bytes.len() as u64 > MAX_FILE_BYTES {
            return Err("MCP registry exceeds its size limit.".into());
        }
        let stored: Inventory =
            serde_json::from_slice(&bytes).map_err(|_| "MCP registry JSON is invalid.")?;
        if stored.version != 1 {
            return Err("Unsupported MCP registry schema; manual migration required.".into());
        }
        validate_all(&stored.servers)?;
        Ok(stored.servers)
    }

    fn write(&self, entries: &[McpServer]) -> Result<(), String> {
        validate_all(entries)?;
        let bytes = serde_json::to_vec_pretty(&Inventory {
            version: 1,
            servers: entries.to_vec(),
        })
        .map_err(|_| "Could not encode MCP registry.")?;
        if bytes.len() as u64 > MAX_FILE_BYTES {
            return Err("MCP registry exceeds its size limit.".into());
        }
        let folder = self.file.parent().ok_or("MCP registry has no directory.")?;
        fs::create_dir_all(folder).map_err(|_| "Could not prepare MCP registry directory.")?;
        let temp = folder.join(format!(".nexora-mcp-{}.tmp", uuid::Uuid::new_v4()));
        let result = (|| {
            let mut options = OpenOptions::new();
            options.write(true).create_new(true);
            #[cfg(unix)]
            {
                use std::os::unix::fs::OpenOptionsExt;
                options.mode(0o600);
            }
            let mut out = options
                .open(&temp)
                .map_err(|_| "Could not create temporary MCP registry.")?;
            out.write_all(&bytes)
                .and_then(|()| out.sync_all())
                .map_err(|_| "Could not save MCP registry.")?;
            drop(out);
            // Never follow a symlink placed at the destination by another process.
            if let Ok(meta) = self.file.symlink_metadata() {
                if !meta.is_file() || meta.file_type().is_symlink() {
                    return Err("MCP registry path changed unexpectedly.".into());
                }
            }
            fs::rename(&temp, &self.file).map_err(|_| "Could not replace MCP registry.")?;
            Ok(())
        })();
        if result.is_err() {
            let _ = fs::remove_file(&temp);
        }
        result
    }

    pub fn list(&self) -> Result<Vec<McpServer>, String> {
        let _guard = self
            .lock
            .lock()
            .map_err(|_| "MCP registry is unavailable.")?;
        self.read()
    }

    pub fn upsert(&self, server: McpServer) -> Result<Vec<McpServer>, String> {
        validate(&server)?;
        let _guard = self
            .lock
            .lock()
            .map_err(|_| "MCP registry is unavailable.")?;
        let mut entries = self.read()?;
        if let Some(index) = entries.iter().position(|item| item.name == server.name) {
            entries[index] = server;
        } else if entries.len() < MAX_SERVERS {
            entries.push(server);
        } else {
            return Err("MCP server limit reached.".into());
        }
        entries.sort_by(|a, b| a.name.cmp(&b.name));
        self.write(&entries)?;
        Ok(entries)
    }

    pub fn remove(&self, name: &str) -> Result<Vec<McpServer>, String> {
        validate_name(name)?;
        let _guard = self
            .lock
            .lock()
            .map_err(|_| "MCP registry is unavailable.")?;
        let mut entries = self.read()?;
        let before = entries.len();
        entries.retain(|item| item.name != name);
        if entries.len() == before {
            return Err("MCP server was not found.".into());
        }
        self.write(&entries)?;
        Ok(entries)
    }

    pub fn set_enabled(&self, name: &str, enabled: bool) -> Result<Vec<McpServer>, String> {
        validate_name(name)?;
        let _guard = self
            .lock
            .lock()
            .map_err(|_| "MCP registry is unavailable.")?;
        let mut entries = self.read()?;
        let server = entries
            .iter_mut()
            .find(|item| item.name == name)
            .ok_or("MCP server was not found.")?;
        server.enabled = enabled;
        self.write(&entries)?;
        Ok(entries)
    }
}

fn validate_name(name: &str) -> Result<(), String> {
    if name.is_empty()
        || name.len() > 64
        || !name.as_bytes()[0].is_ascii_lowercase()
        || !name
            .bytes()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'-' || c == b'_')
    {
        return Err(
            "MCP server name must start with a lowercase letter and use a-z, 0-9, _ or -.".into(),
        );
    }
    Ok(())
}

fn printable(value: &str, max: usize) -> bool {
    value.len() <= max && !value.chars().any(char::is_control)
}

pub(crate) fn valid_credential_ref(server: &str, reference: &str) -> bool {
    let prefix = format!("vault:v1:{server}:");
    reference.strip_prefix(&prefix).is_some_and(|id| {
        id.len() == 36
            && uuid::Uuid::parse_str(id).ok().is_some_and(|uuid| uuid.get_version_num() == 4)
    })
}

pub(crate) fn validate(server: &McpServer) -> Result<(), String> {
    validate_name(&server.name)?;
    if server.credential_ref.as_deref().is_some_and(|reference| {
        !valid_credential_ref(&server.name, reference)
    }) {
        return Err("MCP credential reference must be a server-bound vault:v1 UUID; never store raw secrets.".into());
    }
    if !printable(&server.description, 500) {
        return Err("MCP server description is invalid.".into());
    }
    match &server.transport {
        Transport::Stdio { command, args } => {
            let path = Path::new(command);
            if command.is_empty()
                || !printable(command, 1024)
                || args.len() > MAX_ARGUMENTS
                || args.iter().any(|arg| !printable(arg, MAX_STRING))
                || path.components().any(|c| matches!(c, Component::ParentDir))
                || !(path.is_absolute()
                    || (path.components().count() == 1
                        && command.bytes().all(|c| {
                            c.is_ascii_alphanumeric() || matches!(c, b'-' | b'_' | b'.' | b'+')
                        })))
            {
                return Err("MCP stdio executable or arguments are invalid.".into());
            }
            let executable = path
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_ascii_lowercase();
            if [
                "sh",
                "bash",
                "zsh",
                "cmd",
                "cmd.exe",
                "powershell",
                "powershell.exe",
                "pwsh",
            ]
            .contains(&executable.as_str())
            {
                return Err("MCP stdio shell launchers are not permitted.".into());
            }
            if args.iter().any(|arg| {
                let lowered = arg.to_ascii_lowercase();
                [
                    "--token",
                    "--api-key",
                    "--apikey",
                    "--password",
                    "--secret",
                    "--access-token",
                    "--auth",
                    "--header",
                    "--env",
                    "--proxy-authorization",
                    "authorization:",
                    "proxy-authorization:",
                    "bearer ",
                    "basic ",
                    "x-api-key:",
                    "api_key=",
                    "apikey=",
                    "token=",
                    "password=",
                    "secret=",
                    "client_secret=",
                ]
                .iter()
                .any(|pattern| lowered.contains(pattern))
            }) {
                return Err(
                    "Do not store MCP credentials in arguments; use a secure credential reference."
                        .into(),
                );
            }
        }
        Transport::StreamableHttp { url } => {
            if url.len() > 2048 || !printable(url, 2048) {
                return Err("MCP server URL is invalid.".into());
            }
            let parsed = tauri::Url::parse(url).map_err(|_| "MCP server URL is invalid.")?;
            let hostname = parsed.host_str().ok_or("MCP server URL needs a host.")?;
            if !parsed.username().is_empty()
                || parsed.password().is_some()
                || parsed.query().is_some()
                || parsed.fragment().is_some()
                || !matches!(parsed.scheme(), "https" | "http")
                || (parsed.scheme() == "http"
                    && !["localhost", "127.0.0.1", "[::1]"].contains(&hostname))
            {
                return Err(
                    "MCP HTTP requires HTTPS (except loopback) and no inline credentials or query."
                        .into(),
                );
            }
        }
    }
    Ok(())
}

fn validate_all(entries: &[McpServer]) -> Result<(), String> {
    if entries.len() > MAX_SERVERS {
        return Err("MCP server limit reached.".into());
    }
    let mut names = std::collections::HashSet::new();
    for entry in entries {
        validate(entry)?;
        if !names.insert(&entry.name) {
            return Err("Duplicate MCP server name.".into());
        }
    }
    Ok(())
}

// All registry I/O happens outside the Tauri UI thread. The ordinary webview
// cannot invoke these commands: lib.rs exposes them to "main" only.
#[tauri::command]
pub async fn mcp_registry_list(state: State<'_, Registry>) -> Result<Vec<McpServer>, String> {
    let registry = state.inner().clone();
    tauri::async_runtime::spawn_blocking(move || registry.list())
        .await
        .map_err(|_| "MCP registry worker failed.".to_owned())?
}

#[tauri::command]
pub async fn mcp_registry_upsert(
    state: State<'_, Registry>,
    server: McpServer,
) -> Result<Vec<McpServer>, String> {
    let registry = state.inner().clone();
    tauri::async_runtime::spawn_blocking(move || registry.upsert(server))
        .await
        .map_err(|_| "MCP registry worker failed.".to_owned())?
}

#[tauri::command]
pub async fn mcp_registry_remove(
    state: State<'_, Registry>,
    name: String,
) -> Result<Vec<McpServer>, String> {
    let registry = state.inner().clone();
    tauri::async_runtime::spawn_blocking(move || registry.remove(&name))
        .await
        .map_err(|_| "MCP registry worker failed.".to_owned())?
}

#[tauri::command]
pub async fn mcp_registry_set_enabled(
    state: State<'_, Registry>,
    name: String,
    enabled: bool,
) -> Result<Vec<McpServer>, String> {
    let registry = state.inner().clone();
    tauri::async_runtime::spawn_blocking(move || registry.set_enabled(&name, enabled))
        .await
        .map_err(|_| "MCP registry worker failed.".to_owned())?
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp() -> PathBuf {
        let path =
            std::env::temp_dir().join(format!("nexora-mcp-fixture-{}", uuid::Uuid::new_v4()));
        fs::create_dir(&path).unwrap();
        path
    }

    fn stdio(name: &str) -> McpServer {
        McpServer {
            name: name.into(),
            description: "Example MCP server".into(),
            enabled: false,
            credential_ref: None,
            transport: Transport::Stdio {
                command: "npx".into(),
                args: vec![
                    "-y".into(),
                    "@modelcontextprotocol/server-filesystem".into(),
                ],
            },
        }
    }

    #[test]
    fn persists_and_updates_without_touching_upstream_provider_config() {
        let folder = temp();
        let a = Registry::new(&folder);
        assert!(a.list().unwrap().is_empty());
        a.upsert(stdio("filesystem")).unwrap();
        assert_eq!(Registry::new(&folder).list().unwrap().len(), 1);
        let list = a.set_enabled("filesystem", true).unwrap();
        assert!(list[0].enabled);
        a.upsert(stdio("filesystem")).unwrap();
        assert!(!a.list().unwrap()[0].enabled);
        assert!(a.remove("filesystem").unwrap().is_empty());
        assert!(a.remove("filesystem").is_err());
        fs::remove_dir_all(folder).unwrap();
    }

    #[test]
    fn rejects_invalid_names_and_unknown_json_fields() {
        for name in ["", "Filesystem", "../escape", "a/b", ".git", "a b", "a\nb"] {
            assert!(validate_name(name).is_err(), "{name:?}");
        }
        let encoded = r#"{"name":"x","description":"","enabled":false,"transport":{"kind":"stdio","command":"npx","args":[],"env":{"SECRET":"x"}}}"#;
        assert!(serde_json::from_str::<McpServer>(encoded).is_err());
    }

    #[test]
    fn rejects_secret_args_shell_wrappers_and_traversal() {
        let mut item = stdio("fs");
        for command in ["bash", "powershell.exe", "./tool", "../tool"] {
            item.transport = Transport::Stdio {
                command: command.into(),
                args: vec![],
            };
            assert!(validate(&item).is_err(), "{command}");
        }
        for value in [
            "--api-key=123",
            "Authorization: Bearer secret",
            "--password",
            "--token",
            "--header",
            "--env",
            "x-api-key: top-secret",
            "api_key=top-secret",
            "client_secret=top-secret",
            "Bearer secret",
            "--access-token",
        ] {
            item.transport = Transport::Stdio {
                command: "npx".into(),
                args: vec![value.into()],
            };
            assert!(validate(&item).is_err(), "{value}");
        }
    }

    #[test]
    fn remote_http_requires_tls_and_rejects_embedded_credentials() {
        let mut item = stdio("remote");
        for url in [
            "http://remote.example/mcp",
            "https://user:secret@remote.example/mcp",
            "https://remote.example/mcp?token=secret",
            "file:///etc/passwd",
            "https://remote.example/mcp#frag",
        ] {
            item.transport = Transport::StreamableHttp { url: url.into() };
            assert!(validate(&item).is_err(), "{url}");
        }
        for url in ["https://remote.example/mcp", "http://127.0.0.1:17777/mcp"] {
            item.transport = Transport::StreamableHttp { url: url.into() };
            assert!(validate(&item).is_ok(), "{url}");
        }
    }

    #[test]
    fn corrupt_schema_never_overwrites_existing_data() {
        let folder = temp();
        let reg = Registry::new(&folder);
        fs::write(&reg.file, b"{broken").unwrap();
        assert!(reg.list().is_err());
        assert!(reg.upsert(stdio("safe")).is_err());
        assert_eq!(fs::read(&reg.file).unwrap(), b"{broken");
        fs::remove_dir_all(folder).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn registry_refuses_symlink_targets_and_keeps_private_file_mode() {
        use std::os::unix::{fs::symlink, fs::PermissionsExt};
        let folder = temp();
        let reg = Registry::new(&folder);
        reg.upsert(stdio("safe")).unwrap();
        assert_eq!(
            fs::metadata(&reg.file).unwrap().permissions().mode() & 0o777,
            0o600
        );
        let other = folder.join("victim");
        fs::write(&other, b"unchanged").unwrap();
        fs::remove_file(&reg.file).unwrap();
        symlink(&other, &reg.file).unwrap();
        assert!(reg.upsert(stdio("another")).is_err());
        assert_eq!(fs::read(&other).unwrap(), b"unchanged");
        fs::remove_dir_all(folder).unwrap();
    }

    #[test]
    fn simultaneous_writers_do_not_lose_unrelated_servers() {
        let folder = temp();
        let reg = Registry::new(&folder);
        let handles: Vec<_> = (0..10)
            .map(|i| {
                let copy = reg.clone();
                std::thread::spawn(move || copy.upsert(stdio(&format!("worker{i}"))))
            })
            .collect();
        for handle in handles {
            handle.join().unwrap().unwrap();
        }
        assert_eq!(reg.list().unwrap().len(), 10);
        fs::remove_dir_all(folder).unwrap();
    }

    #[test]
    fn malformed_records_cannot_change_existing_valid_registry() {
        let folder = temp();
        let reg = Registry::new(&folder);
        reg.upsert(stdio("good")).unwrap();
        let before = fs::read(&reg.file).unwrap();
        let mut bad = stdio("bad");
        bad.transport = Transport::StreamableHttp {
            url: "http://metadata.google.internal/computeMetadata/v1".into(),
        };
        assert!(reg.upsert(bad).is_err());
        assert_eq!(fs::read(&reg.file).unwrap(), before);
        fs::remove_dir_all(folder).unwrap();
    }

    #[test]
    fn credential_references_are_scoped_and_never_inline_secrets() {
        let folder = temp();
        let reg = Registry::new(&folder);
        let mut item = stdio("filesystem");
        let reference = format!("vault:v1:filesystem:{}", uuid::Uuid::new_v4());
        item.credential_ref = Some(reference.clone());
        assert!(reg.upsert(item.clone()).is_ok());
        assert_eq!(reg.list().unwrap()[0].credential_ref.as_deref(), Some(reference.as_str()));
        let stored = fs::read_to_string(&reg.file).unwrap();
        assert!(stored.contains("credentialRef"));
        assert!(!stored.contains("api-key"));

        for bad in [
            "password=abc".to_owned(),
            "vault:v1:other:".to_owned() + &uuid::Uuid::new_v4().to_string(),
            "vault:v1:filesystem:not-a-uuid".into(),
            "vault:v1:filesystem:12345678-1234-1234-1234-123456789abc".into(),
        ] {
            item.credential_ref = Some(bad);
            assert!(reg.upsert(item.clone()).is_err());
            assert_eq!(fs::read_to_string(&reg.file).unwrap(), stored);
        }
        fs::remove_dir_all(folder).unwrap();
    }

    #[test]
    fn v1_inventory_without_credential_reference_still_reads_and_serializes() {
        let encoded = serde_json::to_string(&stdio("legacy")).unwrap();
        assert!(!encoded.contains("credentialRef"));
        let decoded: McpServer = serde_json::from_str(&encoded).unwrap();
        assert_eq!(decoded.credential_ref, None);
    }

    #[test]
    fn inventory_has_bounded_count() {
        let folder = temp();
        let reg = Registry::new(&folder);
        for i in 0..MAX_SERVERS {
            reg.upsert(stdio(&format!("server{i}"))).unwrap();
        }
        assert!(reg.upsert(stdio("exceed")).is_err());
        assert_eq!(reg.list().unwrap().len(), MAX_SERVERS);
        fs::remove_dir_all(folder).unwrap();
    }
}
