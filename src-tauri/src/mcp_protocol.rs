//! R2, internal-only MCP discovery/catalog protocol core.
//! No IO, subprocess, sockets, credential access, IPC exports or tool execution.
//! Caller must enforce a verified native operator consent + safe transport BEFORE
//! a real server can be connected. Supports 2026-07-28 stateless and the 2025
//! handshake era without silently confusing the two wire contracts.
#![allow(dead_code)]

use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};

pub const MAX_RPC_FRAME: usize = 128 * 1024;
const MAX_ENTRIES_PER_PAGE: usize = 64;
const MAX_ENTRIES_TOTAL: usize = 256;
const MAX_PAGES: usize = 16;
const MAX_CURSOR_BYTES: usize = 256;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Era {
    Modern20260728,
    Legacy20251125,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Catalog {
    Tools,
    Resources,
    Prompts,
}
impl Catalog {
    fn method(self) -> &'static str {
        match self {
            Self::Tools => "tools/list",
            Self::Resources => "resources/list",
            Self::Prompts => "prompts/list",
        }
    }
    fn key(self) -> &'static str {
        match self {
            Self::Tools => "tools",
            Self::Resources => "resources",
            Self::Prompts => "prompts",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Entry {
    pub kind: Catalog,
    pub name: String,
    pub label: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProtocolFailure {
    InvalidState,
    InvalidFrame,
    Oversize,
    UnexpectedResponse,
    RemoteError,
    UnsupportedProtocol,
    CapabilityUnavailable,
    CatalogLimit,
    CursorCycle,
}

struct Pending {
    id: u64,
    kind: Catalog,
}

#[derive(Default)]
struct CatalogState {
    entries: Vec<Entry>,
    pages: usize,
    next_cursor: Option<String>,
    cursors: HashSet<String>,
    names: HashSet<String>,
    finished: bool,
}

pub struct Inspector {
    era: Era,
    ready: bool,
    poisoned: bool,
    request_id: u64,
    pending: Option<Pending>,
    advertised: HashSet<&'static str>,
    catalogs: HashMap<Catalog, CatalogState>,
}

impl Inspector {
    pub fn new(era: Era) -> Self {
        Self {
            era, ready: false, poisoned: false, request_id: 1,
            pending: None, advertised: HashSet::new(), catalogs: HashMap::new(),
        }
    }

    fn envelope(&self, mut params: Value) -> Value {
        if self.era == Era::Modern20260728 {
            params["_meta"] = json!({
                "io.modelcontextprotocol/protocolVersion":"2026-07-28",
                "io.modelcontextprotocol/clientCapabilities":{},
                "io.modelcontextprotocol/clientInfo":{"name":"nexora-code","version":"0.1.0"}
            });
        }
        params
    }

    pub fn discover_request(&self) -> Value {
        match self.era {
            Era::Modern20260728 => json!({
                "jsonrpc":"2.0","id":1,"method":"server/discover",
                "params":self.envelope(json!({}))
            }),
            Era::Legacy20251125 => json!({
                "jsonrpc":"2.0","id":1,"method":"initialize",
                "params":{"protocolVersion":"2025-11-25","capabilities":{},
                          "clientInfo":{"name":"nexora-code","version":"0.1.0"}}
            }),
        }
    }

    pub fn accept_discovery(&mut self, frame: &[u8]) -> Result<(), ProtocolFailure> {
        if self.poisoned || self.ready || self.pending.is_some() {
            return Err(ProtocolFailure::InvalidState);
        }
        let result = self.parse(frame, 1)?;
        if self.era == Era::Legacy20251125 {
            if result["protocolVersion"] != "2025-11-25" {
                self.poisoned = true;
                return Err(ProtocolFailure::UnsupportedProtocol);
            }
            if !result["serverInfo"]["name"].is_string() {
                self.poisoned = true;
                return Err(ProtocolFailure::InvalidFrame);
            }
        }
        let caps = result["capabilities"].as_object().ok_or_else(|| {
            self.poisoned = true;
            ProtocolFailure::InvalidFrame
        })?;
        for (kind, key) in [(Catalog::Tools, "tools"),
                            (Catalog::Resources, "resources"),
                            (Catalog::Prompts, "prompts")] {
            if caps.contains_key(key) && caps[key].is_object() {
                self.advertised.insert(kind.key());
            }
        }
        self.request_id = 2;
        self.ready = true;
        Ok(())
    }

    pub fn initialized_notification(&self) -> Option<Value> {
        (self.era == Era::Legacy20251125 && self.ready && !self.poisoned)
            .then(|| json!({"jsonrpc":"2.0","method":"notifications/initialized"}))
    }

    pub fn request_page(&mut self, kind: Catalog) -> Result<Value, ProtocolFailure> {
        if !self.ready || self.poisoned || self.pending.is_some() {
            return Err(ProtocolFailure::InvalidState);
        }
        if !self.advertised.contains(kind.key()) {
            return Err(ProtocolFailure::CapabilityUnavailable);
        }
        let state = self.catalogs.entry(kind).or_default();
        if state.finished || state.pages >= MAX_PAGES {
            return Err(ProtocolFailure::CatalogLimit);
        }
        let params = match &state.next_cursor {
            Some(cursor) => json!({"cursor":cursor}),
            None => json!({}),
        };
        let id = self.request_id;
        self.request_id = self.request_id.checked_add(1).ok_or(ProtocolFailure::CatalogLimit)?;
        self.pending = Some(Pending { id, kind });
        Ok(json!({"jsonrpc":"2.0","id":id,"method":kind.method(),
                  "params":self.envelope(params)}))
    }

    pub fn accept_page(&mut self, kind: Catalog, frame: &[u8]) -> Result<bool, ProtocolFailure> {
        let pending = self.pending.as_ref().ok_or(ProtocolFailure::InvalidState)?;
        if self.poisoned || pending.kind != kind {
            return Err(ProtocolFailure::InvalidState);
        }
        let id = pending.id;
        let result = self.parse(frame, id)?;
        let raw = result[kind.key()].as_array().ok_or_else(|| {
            self.poisoned = true;
            ProtocolFailure::InvalidFrame
        })?;
        if raw.len() > MAX_ENTRIES_PER_PAGE {
            self.poisoned = true;
            return Err(ProtocolFailure::CatalogLimit);
        }
        let state = self.catalogs.entry(kind).or_default();
        if raw.len() + state.entries.len() > MAX_ENTRIES_TOTAL {
            self.poisoned = true;
            return Err(ProtocolFailure::CatalogLimit);
        }
        let mut entries = Vec::with_capacity(raw.len());
        let mut new_names = HashSet::new();
        for item in raw {
            let name = item[if kind == Catalog::Resources { "uri" } else { "name" }]
                .as_str().filter(|v| !v.is_empty() && v.len() <= 1024
                    && !v.chars().any(char::is_control))
                .ok_or_else(|| { self.poisoned = true; ProtocolFailure::InvalidFrame })?;
            if state.names.contains(name) || !new_names.insert(name.to_owned()) {
                self.poisoned = true;
                return Err(ProtocolFailure::CatalogLimit);
            }
            let label = item["title"].as_str().unwrap_or(name);
            if label.len() > 1024 || label.chars().any(char::is_control) {
                self.poisoned = true;
                return Err(ProtocolFailure::InvalidFrame);
            }
            if kind == Catalog::Tools
                && !item["inputSchema"].is_object()
            {
                self.poisoned = true;
                return Err(ProtocolFailure::InvalidFrame);
            }
            entries.push(Entry { kind, name: name.to_owned(), label: label.to_owned() });
        }
        let cursor = match result.get("nextCursor") {
            Some(Value::String(value)) if !value.is_empty()
                && value.len() <= MAX_CURSOR_BYTES
                && !value.chars().any(char::is_control) => Some(value.clone()),
            None => None,
            _ => { self.poisoned = true; return Err(ProtocolFailure::InvalidFrame); }
        };
        if let Some(next) = &cursor {
            if !state.cursors.insert(next.clone()) || state.pages + 1 >= MAX_PAGES {
                self.poisoned = true;
                return Err(ProtocolFailure::CursorCycle);
            }
        }
        for entry in entries {
            state.names.insert(entry.name.clone());
            state.entries.push(entry);
        }
        state.pages += 1;
        state.finished = cursor.is_none();
        state.next_cursor = cursor;
        self.pending = None;
        Ok(state.finished)
    }

    pub fn entries(&self, kind: Catalog) -> &[Entry] {
        self.catalogs.get(&kind).map(|state| state.entries.as_slice()).unwrap_or(&[])
    }

    fn parse(&mut self, frame: &[u8], id: u64) -> Result<Value, ProtocolFailure> {
        if frame.len() > MAX_RPC_FRAME {
            self.poisoned = true;
            return Err(ProtocolFailure::Oversize);
        }
        // Never embed untrusted frame/error text in a returned diagnostic.
        let parsed: Value = serde_json::from_slice(frame).map_err(|_| {
            self.poisoned = true;
            ProtocolFailure::InvalidFrame
        })?;
        if !parsed.is_object() || parsed["jsonrpc"] != "2.0" ||
            parsed["id"].as_u64() != Some(id) || parsed.get("method").is_some()
        {
            self.poisoned = true;
            return Err(ProtocolFailure::UnexpectedResponse);
        }
        if parsed.get("error").is_some() {
            self.poisoned = true;
            return Err(ProtocolFailure::RemoteError);
        }
        if !parsed["result"].is_object() {
            self.poisoned = true;
            return Err(ProtocolFailure::InvalidFrame);
        }
        Ok(parsed["result"].clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn response(id: u64, result: Value) -> Vec<u8> {
        serde_json::to_vec(&json!({"jsonrpc":"2.0","id":id,"result":result})).unwrap()
    }

    #[test]
    fn modern_2026_discover_and_three_catalogs_are_read_only_and_bounded() {
        let mut client = Inspector::new(Era::Modern20260728);
        let d = client.discover_request();
        assert_eq!(d["method"], "server/discover");
        assert_eq!(d["params"]["_meta"]["io.modelcontextprotocol/protocolVersion"], "2026-07-28");
        assert_eq!(client.request_page(Catalog::Tools), Err(ProtocolFailure::InvalidState));
        client.accept_discovery(&response(1, json!({"capabilities":{"tools":{},"resources":{},"prompts":{}}}))).unwrap();
        assert!(client.initialized_notification().is_none());
        for kind in [Catalog::Tools, Catalog::Resources, Catalog::Prompts] {
            let q = client.request_page(kind).unwrap();
            assert_eq!(q["method"], kind.method());
            assert_eq!(q["params"]["_meta"]["io.modelcontextprotocol/protocolVersion"], "2026-07-28");
            let id = q["id"].as_u64().unwrap();
            let item = match kind {
                Catalog::Tools => json!({"name":"search","inputSchema":{"type":"object"}}),
                Catalog::Resources => json!({"uri":"resource://safe"}),
                Catalog::Prompts => json!({"name":"analysis"}),
            };
            assert_eq!(client.accept_page(kind, &response(id, json!({kind.key():[item]}))), Ok(true));
            assert_eq!(client.entries(kind).len(), 1);
        }
    }

    #[test]
    fn legacy_initialize_handshake_is_not_confused_with_modern() {
        let mut client = Inspector::new(Era::Legacy20251125);
        let init = client.discover_request();
        assert_eq!(init["method"], "initialize");
        assert!(init["params"].get("_meta").is_none());
        client.accept_discovery(&response(1, json!({
            "protocolVersion":"2025-11-25",
            "serverInfo":{"name":"fixture","version":"1"},
            "capabilities":{"tools":{}}
        }))).unwrap();
        assert_eq!(client.initialized_notification().unwrap()["method"], "notifications/initialized");
        assert_eq!(client.request_page(Catalog::Resources), Err(ProtocolFailure::CapabilityUnavailable));
        let query = client.request_page(Catalog::Tools).unwrap();
        assert!(query["params"].get("_meta").is_none());
    }

    #[test]
    fn pagination_cursor_cycle_and_wrong_request_id_fail_closed() {
        let mut client = Inspector::new(Era::Modern20260728);
        client.accept_discovery(&response(1, json!({"capabilities":{"tools":{}}}))).unwrap();
        let request = client.request_page(Catalog::Tools).unwrap();
        let id = request["id"].as_u64().unwrap();
        assert_eq!(client.accept_page(Catalog::Tools, &response(id + 1, json!({"tools":[]}))),
            Err(ProtocolFailure::UnexpectedResponse));
        assert_eq!(client.request_page(Catalog::Tools), Err(ProtocolFailure::InvalidState));

        let mut client = Inspector::new(Era::Modern20260728);
        client.accept_discovery(&response(1, json!({"capabilities":{"prompts":{}}}))).unwrap();
        let q1 = client.request_page(Catalog::Prompts).unwrap();
        client.accept_page(Catalog::Prompts, &response(q1["id"].as_u64().unwrap(),json!({
            "prompts":[{"name":"one"}],"nextCursor":"next"
        }))).unwrap();
        let q2 = client.request_page(Catalog::Prompts).unwrap();
        assert_eq!(q2["params"]["cursor"], "next");
        assert_eq!(client.accept_page(Catalog::Prompts, &response(q2["id"].as_u64().unwrap(),json!({
            "prompts":[{"name":"two"}],"nextCursor":"next"
        }))), Err(ProtocolFailure::CursorCycle));
    }

    #[test]
    fn malformed_oversized_tool_pages_and_duplicate_entries_are_rejected() {
        let mut c = Inspector::new(Era::Modern20260728);
        c.accept_discovery(&response(1,json!({"capabilities":{"tools":{}}}))).unwrap();
        let _id = c.request_page(Catalog::Tools).unwrap()["id"].as_u64().unwrap();
        assert_eq!(c.accept_page(Catalog::Tools, &vec![b'x'; MAX_RPC_FRAME + 1]), Err(ProtocolFailure::Oversize));
        let mut c = Inspector::new(Era::Modern20260728);
        c.accept_discovery(&response(1,json!({"capabilities":{"tools":{}}}))).unwrap();
        let id = c.request_page(Catalog::Tools).unwrap()["id"].as_u64().unwrap();
        assert_eq!(c.accept_page(Catalog::Tools, &response(id,json!({
            "tools":[{"name":"a","inputSchema":{"type":"object"}},{"name":"a","inputSchema":{"type":"object"}}]
        }))), Err(ProtocolFailure::CatalogLimit));
        assert!(c.entries(Catalog::Tools).is_empty());
        let mut c = Inspector::new(Era::Modern20260728);
        c.accept_discovery(&response(1,json!({"capabilities":{"tools":{}}}))).unwrap();
        let id = c.request_page(Catalog::Tools).unwrap()["id"].as_u64().unwrap();
        assert_eq!(c.accept_page(Catalog::Tools, &response(id,json!({"tools":[{"name":"a"}]}))),
            Err(ProtocolFailure::InvalidFrame));
    }

    #[test]
    fn remote_error_is_redacted_and_parser_does_not_execute_or_log_input() {
        let mut client=Inspector::new(Era::Modern20260728);
        let error=br#"{"jsonrpc":"2.0","id":1,"error":{"code":-32603,"message":"PASSWORD=in-memory-secret"}}"#;
        assert_eq!(client.accept_discovery(error),Err(ProtocolFailure::RemoteError));
        assert_eq!(client.request_page(Catalog::Tools),Err(ProtocolFailure::InvalidState));
    }
}
